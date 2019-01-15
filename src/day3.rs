use serde

#[derive(Deserialize, Eq, PartialEq, Ord, PartialOrd)]
struct Claim {
    id: u32,
    pos: (u32, u32),
    dim: (u32, u32),
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> &[Claim] {
    input.lines.map(|&line| scan!("#{} @ {},{}: {}x{}" <- line).unwrap()).collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Claim]) -> u32 {
    unimplemented!()
}