use std::fmt::format;


fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);

}

fn part2(input: &str)  -> String{
    let digits = [
        ("one", "o1e"), 
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
        ("zero", "z0o")];

    input
    .lines()
    .map(|line| {

        let mut newline = String::from(line);
        for (from, to) in digits{

        newline = newline.replace(from, to);

        }

        let nums = newline.chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>();

        format!("{}{}", nums.chars().next().unwrap(), nums.chars().last().unwrap()).parse::<u32>().unwrap()
        
    }).sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_works() {
        let result = part2("two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen");
        assert_eq!(result, "281");
    }
}