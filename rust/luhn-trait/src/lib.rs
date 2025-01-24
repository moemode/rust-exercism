use std::fmt::Display;

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<D: Display> Luhn for D {
    fn valid_luhn(&self) -> bool {
        self.to_string()
            .chars()
            .filter(|c| !c.is_whitespace())
            .rev()
            .try_fold((0, 0), |(count, sum), val| {
                val.to_digit(10).map(|mut d| {
                    if count % 2 == 1 {
                        d *= 2;
                    }
                    if d > 9 {
                        d -= 9;
                    }
                    (count + 1, sum + d)
                })
            })
            .map_or(false, |(count, sum)| count > 1 && sum % 10 == 0)
    }
}
