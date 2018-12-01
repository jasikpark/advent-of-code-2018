//use std::collections::HashSet;
//use std::iter::repeat;
use fnv::FnvHashSet;

// Nice and easy, lemon squeezy.
#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| {
            let num = l.trim().parse();
            num.unwrap()
        }).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

// So, I think that I probably need to take the current frequency each time and compare it to the
// previous ones. Ooh yeah, I figured it out - we can just append each element to a new HashSet<i32>
// that will now contain all previous sums.
#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    let mut seen: FnvHashSet<i32> = FnvHashSet::default();
    let mut index: usize = 0;
    while !seen.contains(&sum) {
        seen.insert(sum);
        sum += input[index];
        index += 1;
        if index >= input.len() {
            index = 0;
        }
    }
    sum
}