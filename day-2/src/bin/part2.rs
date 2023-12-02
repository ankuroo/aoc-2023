fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);

}

fn part2(input: &str)  -> String{

    input
        .lines()
        .map(
            |line| {

                let split_result = line.split(": ").collect::<Vec<&str>>();
                let split_result = split_result[split_result.len() - 1].replace("; ", ", ");
                let draws = split_result.split(", ").collect::<Vec<&str>>();

                let red_min = draws.iter().filter_map(|draw| {
                    let count = draw.split(" ").collect::<Vec<&str>>()[0];
                    let colour = draw.split(" ").collect::<Vec<&str>>()[1];

                    if colour == "red" {
                        Some(count.parse::<u32>().unwrap())
                    } else {
                        None
                    }
                }).max().unwrap();

                let green_min = draws.iter().filter_map(|draw| {
                    let count = draw.split(" ").collect::<Vec<&str>>()[0];
                    let colour = draw.split(" ").collect::<Vec<&str>>()[1];

                    if colour == "green" {
                        Some(count.parse::<u32>().unwrap())
                    } else {
                        None
                    }
                }).max().unwrap();

                let blue_min = draws.iter().filter_map(|draw| {
                    let count = draw.split(" ").collect::<Vec<&str>>()[0];
                    let colour = draw.split(" ").collect::<Vec<&str>>()[1];

                    if colour == "blue" {
                        Some(count.parse::<u32>().unwrap())
                    } else {
                        None
                    }
                }).max().unwrap();


                red_min  * green_min * blue_min

            }
        ).sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_works() {
        let result = part2("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, "2286");
    }
}
