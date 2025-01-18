use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '\'')
        .filter_map(|word| {
            let candidate = word.trim().trim_start_matches('\'').trim_end_matches('\'');
            if candidate.is_empty() {
                None
            } else {
                Some(candidate.to_lowercase())
            }
        }
        )
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(word).or_insert(0) += 1;
            acc
        })
}
