pub struct Luhn {
    digits: Option<Vec<u8>>,
}

impl Luhn {
    fn new_str(input: &str) -> Self {
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

impl From<&str> for Luhn {
    fn from(input: &str) -> Self {
        Luhn::new_str(input)
    }
}

impl From<String> for Luhn {
    fn from(input: String) -> Self {
        Luhn::new_str(input.as_str())
    }
}

macro_rules! luhn_from {
    ($($ty:ty),+ $(,)?) => {
        struct DigitIterator<T> {
            value: T,
        }
        $(
            impl Iterator for DigitIterator<$ty> {
                type Item = u8;
                fn next(&mut self) -> Option<Self::Item> {
                    if self.value == 0 {
                        None
                    } else {
                        let digit = self.value % 10;
                        self.value /= 10;
                        Some(digit as u8)
                    }
                }
            }

            impl From<$ty> for Luhn {
                fn from(input: $ty) -> Self {
                    Luhn {
                        digits: Some(DigitIterator { value: input }.collect::<Vec<_>>().iter().copied().rev().collect()),
                    }
                }
            }
        )+
    };
}

luhn_from!(usize, u8, u16, u32, u64, u128);

/*
/// implementation for number types, unfortunately incompatible with the &str implementation
impl<N> From<N> for Luhn {
    fn from(input: N) -> Self {
        Luhn {
            digits: Some(DigitIterator { value: input }.collect()),
        }
    }
}

struct DigitIterator<N> {
    value: N,
}

impl<N> Iterator for DigitIterator<N> {
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
