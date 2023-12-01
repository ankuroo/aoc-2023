use std::fmt::format;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);

}

fn part1(input: &str)  -> String{
    input
    .lines()
    .map(|line| {
        let nums = line.chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>();

        format!("{}{}", nums.chars().next().unwrap(), nums.chars().last().unwrap()).parse::<u32>().unwrap()
        
    }).sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let result = part1("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");
        assert_eq!(result, "142");
    }
}