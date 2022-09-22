use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut counts = nucleotide_counts(dna)?;
    counts.remove(&nucleotide).ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<char, usize> = ['A', 'C', 'G', 'T']
    .iter()
    .map(|n| (*n, 0))
    .collect();

    for c in dna.chars() {
        counts.get_mut(&c).map(|count| *count += 1).ok_or(c)?
    }
    Ok(counts)
}