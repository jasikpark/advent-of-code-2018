use std::collections::HashMap;

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut words = Vec::new();
    for word in input.lines() {
        words.push(word.chars().fold(HashMap::new(), |mut char_counts, c| {
            let count: &mut usize = char_counts.entry(c).or_insert(0);
            *count += 1;
            char_counts
        }));
    }
    let mut twos: u32 = 0;
    let mut threes: u32 = 0;
    for counts in words.iter() {
        if counts.iter().filter(|(_, count)| **count == 2).count() > 0 {
            twos += 1;
        }
        if counts.iter().filter(|(_, count)| **count == 3).count() > 0 {
            threes += 1;
        }
    }
    twos * threes
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> String {
    input
        .lines()
        .map(|word| {
            input.lines().fold("".to_string(), |most_similar_id, w| {
                if w == word {
                    return most_similar_id;
                }
                let (processed_word, _): (String, String) = w
                    .chars()
                    .zip(word.chars())
                    .filter(|(c1, c2)| c1 == c2)
                    .unzip();
                if most_similar_id.len() < processed_word.len() {
                    processed_word
                } else {
                    most_similar_id
                }
            })
        })
        .fold("".to_string(), |most_similar_id, w| {
            if most_similar_id.len() < w.len() {
                w.to_string()
            } else {
                most_similar_id
            }
        })
}