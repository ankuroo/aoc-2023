use std::{collections::{HashMap, HashSet}};

#[derive(PartialEq, Clone, Debug)]
enum Character {
    Number(u32),
    Symbol(char),
    Nothing
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);

}

fn part2(input: &str)  -> String{


    let char_map: Vec<Vec<Character>> = input.lines()
    .fold(vec![], |mut acc, line| {

        let line_chars: Vec<Character> = line.chars().map(
            |char|
            {
                match char {
                    '.' => Character::Nothing,
                    c if char.is_digit(10) => Character::Number(c.to_digit(10).unwrap()),
                    c => Character::Symbol(c)
                }
            }
        ).collect::<Vec<Character>>();

        acc.push(line_chars);
        acc
        
    });

    let mut numbers:HashMap<(u32, u32), u32> = HashMap::new();

    for (l_idx, l) in char_map.iter().enumerate() {
        let mut _queried_results: HashMap<usize, u32> = HashMap::new();

        for (c_idx, c) in l.iter().enumerate() {
            if !_queried_results.contains_key(&c_idx) {
                if let Character::Number(num) = c {
                    let mut digits : Vec<u32> = vec![*num];
                    let mut next_c_idx = c_idx + 1;
                    _queried_results.insert(c_idx, *num);

                    while next_c_idx < l.len() {
                        if !_queried_results.contains_key(&next_c_idx){
                            if let Character::Number(next_digit) = l[next_c_idx] {
                                digits.push(next_digit);
                                _queried_results.insert(next_c_idx, next_digit);

                                if next_c_idx == l.len() - 1 {
                                numbers.insert((l_idx.try_into().unwrap(), c_idx.try_into().unwrap()), digits.iter().fold(0, |acc, digit| acc * 10 + digit));
                                }

                            } else {
                                numbers.insert((l_idx.try_into().unwrap(), c_idx.try_into().unwrap()), digits.iter().fold(0, |acc, digit| acc * 10 + digit));
                                break;

                            }
                        }
                        next_c_idx+=1;
                    }
                }
            }
        }
    }

    let mut symbols:HashMap<(usize, usize), char> = HashMap::new();

    for (l_idx, l) in char_map.iter().enumerate() {
        for (c_idx, c) in l.iter().enumerate() {
            if let Character::Symbol(symbol) = c {
                symbols.insert((l_idx, c_idx), *symbol);

            }
        }
    }

    const NEIGHBOURS: [(i32, i32); 8] = [(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)];

    symbols.iter().filter_map(|(&(x,y), _)| {

        let mut int_starts: HashSet<(usize, usize)> = HashSet::new();

        NEIGHBOURS.iter().for_each(|(nx, ny)| {
            
            let l_idx = nx + x as i32;
            let mut c_idx = ny + y as i32;

            let mut found_start = false;

            if !(l_idx < 0 || c_idx < 0) {
                let mut start:(usize, usize);

                if let Character::Number(_) = char_map[l_idx as usize][c_idx as usize] {
                    while !found_start {
                        c_idx -= 1;

                        if c_idx >= 0 {
                            if let Character::Number(_) = char_map[l_idx as usize][c_idx as usize] {
                                continue;
                            } else {
                                found_start = true;
                                start = (l_idx as usize, (1 + c_idx) as usize);

                            }
                        } else {
                            found_start = true;
                            start = (l_idx as usize, (1 + c_idx) as usize);

                        }
                        int_starts.insert(start);

                    }
                }
            }
        });

        match int_starts.len() {
            2 => Some(int_starts.iter().map(|&(x,y)| numbers.get(&(x as u32, y as u32)).unwrap()).product::<u32>()),
            _ => None
        }

    }).sum::<u32>().to_string()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_works() { let result = part2("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");
        assert_eq!(result, "467835");
    }
}
