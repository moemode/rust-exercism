use std::fmt::Display;

pub struct Luhn {
    digits: Option<Vec<u8>>,
}

impl Luhn {
    fn new(input: &str) -> Self {
        Luhn {
            digits: input
                .chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| c.to_digit(10))
                .map(|d| d.map(|d| d as u8))
                .collect(),
        }
    }

    fn luhn_sum(digits: &[u8]) -> u32 {
        digits
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &d)| {
                if i % 2 == 1 {
                    let doubled = d * 2;
                    if doubled > 9 {
                        doubled - 9
                    } else {
                        doubled
                    }
                } else {
                    d
                }
            })
            .map(u32::from)
            .sum()
    }

    pub fn is_valid(&self) -> bool {
        self.digits
            .as_ref()
            .filter(|digits| digits.len() > 1)
            .map(|digits| Self::luhn_sum(digits) % 10 == 0)
            .unwrap_or(false)
    }
}

impl<S: Display> From<S> for Luhn {
    fn from(input: S) -> Self {
        Luhn::new(input.to_string().as_str())
    }
}

/*
/// implementation for number types, unfortunately incompatible with the &str implementation
impl<N> From<N> for Luhn
where
    N: Integer + Copy + From<u8> + TryInto<u8> + DivAssign + Rem<Output = N> + Zero,
{
    fn from(input: N) -> Self {
        Luhn {
            digits: Some(DigitIterator { value: input }.collect()),
        }
    }
}

struct DigitIterator<N> {
    value: N,
}

impl<N> Iterator for DigitIterator<N>
where
    N: Integer + Copy + From<u8> + TryInto<u8> + DivAssign + Rem<Output = N> + Zero,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value.is_zero() {
            None
        } else {
            let digit = self.value % N::from(10);
            self.value /= N::from(10);
            digit.try_into().ok()
        }
    }
}
*/
