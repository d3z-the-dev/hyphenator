const VOWELS: &str = "aeiouy";
const CONSONANTS: &str = "bcdfghjklmnpqrstvwxz";

pub fn split(word: &str) -> Vec<Vec<String>> {
    let validate = |syllable: &str| -> bool {
        let (mut vowels, mut consonants) = (false, false);
        for c in syllable.chars().map(|c| c.to_ascii_lowercase()) {
            vowels |= VOWELS.contains(c);
            consonants |= CONSONANTS.contains(c);
            if vowels && consonants {
                return true;
            }
        }
        false
    };

    let mut results = Vec::new();
    let mut stack: Vec<(usize, Vec<String>)> = vec![(0, vec![])];

    while let Some((start, state)) = stack.pop() {
        if start >= word.len() {
            results.push(state);
            continue;
        }
        for length in 2..=4 {
            if start + length <= word.len() {
                let syllable = &word[start..start + length];
                if validate(syllable) {
                    let mut updated = state.clone();
                    updated.push(syllable.to_string());
                    stack.push((start + length, updated));
                }
            }
        }
    }

    results.sort();
    results.dedup();
    results
}