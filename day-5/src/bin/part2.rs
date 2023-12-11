struct Transform {
    range_start: u32,
    range_end: u32,
    difference: i32
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);

}

fn get_map(input: &str) -> Vec<Transform> {

    input.split(":\n").collect::<Vec<&str>>()[1].lines().fold(vec![], |mut acc, line| {
        let dest = line.trim().split_whitespace().collect::<Vec<&str>>()[0].parse::<u32>().unwrap();
        let source = line.trim().split_whitespace().collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let range = line.trim().split_whitespace().collect::<Vec<&str>>()[2].parse::<u32>().unwrap();
        acc.push(Transform{range_start: source, range_end: source.wrapping_add(range-1), difference: dest.wrapping_sub(source) as i32});
        acc

    })

}

fn source_to_dest(input: u32, map: &Vec<Transform>) -> u32 {

    let mut difference = 0;
    for transform in map {
        if input >= transform.range_start && input <= transform.range_end {
            difference = transform.difference;
            break;
        } 
    }

    match difference >= 0 {
        true => input.wrapping_add(difference as u32),
        false => input.wrapping_sub((difference * -1) as u32)
    }

}

fn part2(input: &str) -> String {

    let input = input.replace("\r", "");
    let sections = input.split("\n\n").collect::<Vec<&str>>();

    let seed_ranges:Vec<(u32,u32)> = sections[0].split(": ").collect::<Vec<&str>>()[1].split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>().chunks(2).step_by(2).fold(vec![], |mut acc, range| {
        acc.push((range[0], range[0] + range[1] - 1));
        acc
    });

    let seeds:Vec<u32> = seed_ranges.iter().fold(vec![], |mut acc, range| {
        for seed in range.0..range.1 {
            acc.push(seed);
        }
        acc
    });

    let seed_to_soil = get_map(sections[1]);
    let soil_to_fert = get_map(sections[2]);
    let fert_to_water = get_map(sections[3]);
    let water_to_light = get_map(sections[4]);
    let light_to_temp = get_map(sections[5]);
    let temp_to_humidity = get_map(sections[6]);
    let humidity_to_loc = get_map(sections[7]);

    let locs = seeds.iter().map(|&seed| {

        let soil = source_to_dest(seed, &seed_to_soil);
        let fert = source_to_dest(soil, &soil_to_fert);
        let water = source_to_dest(fert, &fert_to_water);
        let light = source_to_dest(water, &water_to_light);
        let temp = source_to_dest(light, &light_to_temp);
        let humidity = source_to_dest(temp, &temp_to_humidity);
        let location = source_to_dest(humidity, &humidity_to_loc);

        location

    }).collect::<Vec<u32>>();

    locs.iter().min().unwrap().to_string()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_works() {
        let result = part2("seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4");
        assert_eq!(result, "46");
    }
}
