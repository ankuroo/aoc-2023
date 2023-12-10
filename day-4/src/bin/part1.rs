use std::collections::{HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);

}

fn part1(input: &str) -> String{

    input
    .lines().map(|line| {
        let card_set = line.split(": ").collect::<Vec<&str>>()[1];
        let hand: Vec<&str> = card_set.split(" | ").collect::<Vec<&str>>()[1].split_whitespace().collect();
        let winners: HashSet<&str> = card_set.split(" | ").collect::<Vec<&str>>()[0].split_whitespace().collect::<HashSet<&str>>();

        let matches = hand.iter().fold(-1, |acc:i32, num| {
            acc + winners.contains(num) as i32
        });

        match matches {
            -1 => 0,
            _ => 2_i32.pow(matches as u32) as u32
        }
    }).sum::<u32>().to_string()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let result = part1("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(result, "13");
    }
}
