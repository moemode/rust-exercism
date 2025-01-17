struct Node<T> {
    value: T,
    next: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

pub struct SimpleLinkedList<T> {
    head: Link<T>,
    len: usize
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, _element: T) {
        let h = self.head.take();
        let n = Node { value: _element, next: h };
        self.head = Some(Box::new(n));
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let old_head = self.head.take();
        old_head.map(|old_head| {
            if let Some(new_head) = old_head.next {
                self.head = Some(new_head);
            }
            self.len -= 1;
            old_head.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(node) => Some(&node.value),
            None => None
        }
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut prev = None;
        let mut current = self.head.take();
        while let Some(mut cur) = current {
            let next = cur.next.take();
            cur.next = prev;
            prev = Some(cur);
            current = next;
        }
        SimpleLinkedList { head: prev, len: self.len }
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut l = Self::new();
        for v in _iter {
            l.push(v);
        }
        l
    }
}

pub struct SimpleLinkedListIter<T> (SimpleLinkedList<T>);

impl<T> Iterator for SimpleLinkedListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> IntoIterator for SimpleLinkedList<T> {
    type Item = T;
    type IntoIter = SimpleLinkedListIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        SimpleLinkedListIter(self)
    }
}


// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = vec![];
        let mut l = _linked_list.rev();
        while let Some(v) = l.pop() {
            vec.push(v);
        };
        vec
    }
}

