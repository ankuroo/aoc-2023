use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);

}

fn part1(input: &str)  -> String{

    let limits: HashMap<&str, u32> = HashMap::from([("red",12), ("green",13), ("blue",14)]);

    input
        .lines()
        .filter_map(
            |line| {
                let split_result = line.split(": ").collect::<Vec<&str>>();
                let game_no = split_result[0].replace("Game ", "");
                let draws = split_result[split_result.len()-1].split("; ");

                let mut possible = true;

                for draw in draws {
                    let individual_colours = draw.split(", ");
                    for indi_col in individual_colours {
                        let item = indi_col.split(" ").collect::<Vec<&str>>();
                        let count = item[0].parse::<u32>().unwrap();
                        let colour = item[item.len()-1];

                        if count > limits[colour] {
                            possible = false;
                            break;
                        }
                    }

                    if !possible {
                        break;
                    }
                }

                if possible {
                    Some(game_no.parse::<u32>().unwrap())
                } else {
                    None
                }
            }
        ).sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let result = part1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, "8");
    }
}
