use std::collections::HashMap;

fn letter_frequency(input: &[&str]) -> HashMap<char, usize> {
    input
        .iter()
        .flat_map(|&word| word.chars())
        .filter(|l| l.is_alphabetic())
        .flat_map(|l| l.to_lowercase())
        .fold(HashMap::new(), |mut acc, letter| {
            *acc.entry(letter).or_default() += 1;
            acc
        })
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    std::thread::scope(|s| {
        let mut handles = vec![];
        for l in input.chunks(input.len() / worker_count + 1) {
            handles.push(s.spawn(|| letter_frequency(l)));
        }
        let mut all_freqs = HashMap::new();
        for h in handles {
            let freqs = h.join().unwrap();
            for (l, count) in freqs {
                *all_freqs.entry(l).or_default() += count;
            }
        }
        all_freqs
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let input: [&str; 0] = [];
        let worker_count = 4;
        let expected: HashMap<char, usize> = HashMap::new();
        assert_eq!(frequency(&input, worker_count), expected);
    }
}
