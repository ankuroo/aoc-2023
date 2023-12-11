fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);

}

fn part1(input: &str) -> String {

    let times = input.lines().collect::<Vec<&str>>()[0].split(":").collect::<Vec<&str>>()[1].split_whitespace().fold(vec![], |mut acc, time| {acc.push(time.parse::<u32>().unwrap());acc});
    let distances = input.lines().collect::<Vec<&str>>()[1].split(":").collect::<Vec<&str>>()[1].split_whitespace().fold(vec![], |mut acc, distance| {acc.push(distance.parse::<u32>().unwrap());acc});

    times.iter().zip(distances).fold(1, |mut acc, (&time, distance)| {

        let mut beat_start = -1;
        let mut beat_end = -1;

        for time_spent in 0..=time {

            let time_remaining = time - time_spent;
            let attempt_dist = time_spent * time_remaining;
            let attempt_reverse_dist = time_remaining * time_spent;

            if attempt_dist > distance {
                if beat_start == -1 {
                    beat_start = time_spent as i32;
                }
            }

            if attempt_reverse_dist > distance {
                if beat_end == -1 {
                    beat_end = time_remaining as i32;
                }
            }
                
            if beat_start != -1 && beat_end != -1 {
                break;
            }
        }

        acc *= beat_end - beat_start + 1;
        acc

    }).to_string()


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let result = part1("Time:      7  15   30
Distance:  9  40  200
");
        assert_eq!(result, "288");
    }
}
