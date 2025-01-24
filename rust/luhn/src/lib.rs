/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .try_fold((0, 0), |(count, sum), val| {
            val.to_digit(10)
                .map(|d| if count % 2 == 1 { 2 * d } else { d })
                .map(|d| if d > 9 { d - 9 } else { d })
                .map(|d| (count + 1, sum + d))
        })
        .map_or(false, |(count, sum)| count > 1 && sum % 10 == 0)
}
