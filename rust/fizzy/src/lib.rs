// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

use std::{fmt::Display, ops::Rem};

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    matcher: fn(T) -> bool,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new<S: Display>(_matcher: fn(T) -> bool, _subs: S) -> Matcher<T> {
        Matcher {
            matcher: _matcher,
            subs: _subs.to_string(),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T: Display + Clone> Fizzy<T> {
    pub fn new() -> Self {
        Fizzy { matchers: vec![] }
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, _matcher: Matcher<T>) -> Self {
        self.matchers.push(_matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I: Iterator<Item = T>>(self, _iter: I) -> impl Iterator<Item = String> {
        _iter.map(move |t| {
            let matching_matchers = self
                .matchers
                .iter()
                .filter(|matcher| (matcher.matcher)(t.clone()));
            let sub: String = matching_matchers
                .map(|matcher| matcher.subs.clone())
                .collect();
            if sub.is_empty() {
                t.to_string()
            } else {
                sub
            }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T: Display + Clone + Rem<Output = T> + From<u8> + PartialEq>() -> Fizzy<T> {
    Fizzy::new()
        .add_matcher(Matcher::new(|t| t % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|t| t % 5.into() == 0.into(), "buzz"))
}
