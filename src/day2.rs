use std::collections::HashMap;

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let chars_count_iter: () = input
        .lines()
        .map(|word: &str| {
            word.chars().fold(HashMap::new(), |mut char_counts, c| {
                let count: &mut usize = char_counts.entry(c).or_insert(1);
                *count += 1;
                char_counts
            })
        })
        .iter()
        .map(|char_counts: HashMap<char, usize>| char_counts.iter().filter(&|_, count| count == 2));
    3
}
