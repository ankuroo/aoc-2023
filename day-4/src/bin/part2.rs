use std::collections::{HashSet, HashMap};

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);

}

fn part2(input: &str) -> String{

    let mut post_win_cards: HashMap<u32, u32> = HashMap::new();

    input.lines().for_each(|card| {

        let card_num = card.split(": ").collect::<Vec<&str>>()[0].replace("Card ", "").trim().parse::<u32>().unwrap();
        *post_win_cards.entry(card_num).or_insert(0) += 1;
        let card_set = card.split(": ").collect::<Vec<&str>>()[1];
        let hand: Vec<&str> = card_set.split(" | ").collect::<Vec<&str>>()[1].split_whitespace().collect();
        let winners: HashSet<&str> = card_set.split(" | ").collect::<Vec<&str>>()[0].split_whitespace().collect::<HashSet<&str>>();

        let current_card_count = *post_win_cards.get(&card_num).unwrap();

        hand.into_iter().fold(card_num, |mut acc, val| {
            let found  = winners.contains(val) as u32;
            acc += found;
            *post_win_cards.entry(acc).or_insert(0) += found * current_card_count;
            acc

        });

    });

    // dbg!(&post_win_cards);

    post_win_cards.values().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_works() {
        let result = part2("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(result, "30");
    }
}
