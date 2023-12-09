use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug)]
enum Character {
    Number(u32),
    Symbol(char),
    Nothing
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);

}

fn part1(input: &str) -> String{

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

    // let mut numbers:Vec<(u32, u32, u32)> = vec![];
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
                                // numbers.push((l_idx.try_into().unwrap(), c_idx.try_into().unwrap(), digits.iter().fold(0, |acc, digit| acc * 10 + digit)));
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

    for ((x,y),n) in numbers.iter() {
        println!("({x},{y}),{n}");
    }

    println!("");

    for ((x,y),n) in symbols.iter() {
        println!("({x},{y}),{n}");
    }

    println!("");

    numbers.iter().filter_map(|((x,y),n)| {

        let mut neighbours:Vec<(i32, i32)> = vec![(-1,-1), (0,-1), (1, -1)];

        for i in 0..n.to_string().len() {

            neighbours.push((-1,i as i32));
            neighbours.push((1,i as i32));

        }

        neighbours.push((-1,n.to_string().len().try_into().unwrap()));
        neighbours.push((0,n.to_string().len().try_into().unwrap()));
        neighbours.push((1,n.to_string().len().try_into().unwrap()));

        let mut sum = 0;

        for (x_offset,y_offset) in neighbours{

            let neighbour_x = x_offset + *x as i32;
            let neighbour_y = y_offset + *y as i32;

            if !(neighbour_x < 0 || neighbour_y < 0) {
                if symbols.contains_key(&(neighbour_x.try_into().unwrap(), neighbour_y.try_into().unwrap())) {
                    sum += n;
                    break;

                }
            }
        }

        Some(sum)

    }).sum::<u32>().to_string()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let result = part1("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");
        assert_eq!(result, "4361");
    }
}
