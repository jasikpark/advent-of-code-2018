use std::collections::HashMap;

#[aoc_generator(day2)]
pub fn input_generator(input: &'static str) -> Vec<&str> {
    input.lines().collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: Vec<&str>) -> i32 {
    let chars_count_iter = input
        .iter()
        .map(|word| count_chars(word))
        .for_each(|char_counts| char_counts.filter(&|_, count| count == 2));
    3
}

fn count_chars(input: &str) -> HashMap<char, usize> {
    input.chars().fold(HashMap::new(), |mut char_counts, c| {
        let count: &mut usize = char_counts.entry(c).or_insert(1);
        *count += 1;
        char_counts
    })
}
