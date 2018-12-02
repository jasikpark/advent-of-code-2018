use std::collections::HashSet;
//use std::iter::repeat;
extern crate fnv;
use fnv::FnvHashSet;

extern crate hashbrown;

use hashbrown::HashSet as HashbrownHashSet;

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
#[aoc(day1, part2, StdCollections)]
pub fn solve_part2_std(input: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    let mut seen: HashSet<i32> = HashSet::new();
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

#[aoc(day1, part2, Fnv)]
pub fn solve_part2_fnv(input: &[i32]) -> i32 {
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

#[aoc(day1, part2, Hashbrown)]
pub fn solve_part2_hashbrown(input: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    let mut seen: HashbrownHashSet<i32> = HashbrownHashSet::new();
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