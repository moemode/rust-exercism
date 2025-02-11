use std::collections::HashMap;

fn is_nucleotide(c: char) -> bool {
    c == 'A' || c == 'C' || c == 'G' || c == 'T'
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_nucleotide(nucleotide) {
        return Err(nucleotide);
    }
    let mut c: usize = 0;
    for d in dna.chars() {
        if !is_nucleotide(d) {
            return Err(d);
        }
        if d == nucleotide {
            c += 1;
        }
    }
    Ok(c)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);
    for c in dna.chars() {
        counts.get_mut(&c).map(|count| *count += 1).ok_or(c)?;
    }
    Ok(counts)
}
