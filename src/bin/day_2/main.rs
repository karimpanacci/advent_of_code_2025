use std::ops::Range;
use std::str::FromStr;

fn parse_ranges(ranges: &str) -> Vec<Range<u64>> {
    ranges.split(",").map(|range| {
        let mut range = range.split("-");
        let lower = range.next().expect("lower bound not found");
        let upper = range.next().expect("upper bound not found");

        let lower = u64::from_str(lower).expect("lower is not a positive number");
        let upper = u64::from_str(upper).expect("upper is not a positive number");
        lower..(upper + 1)
    }).collect()
}

// Return true if id is not valid (the goal of this puzzle)
fn is_invalid_id(id: &str) -> bool {
    if id.len() % 2 != 0 {
        return false
    }
    let id_splitted = id.split_at(id.len() / 2);
    
    id_splitted.0 == id_splitted.1
}

fn is_invalid_id_part_two(id: &str) -> bool {
    if id.len() <= 1 {
        return false
    }

    for n in 2..=id.len() {
        if id.len() % n != 0 {
            continue
        }

        let mut splitted_strings: Vec<String> = vec![];

        let mut id_chars: Vec<_> = id.chars().collect();
        for _ in 0..n {
            let splitted_string = id_chars.drain(..id.len()/n);
            splitted_strings.push(splitted_string.collect())
        }

        let is_invalid = splitted_strings.iter().all(|s| *s == splitted_strings[0]);
        if is_invalid {
            return true
        }
    }

    false
}

fn run(ranges: &str, is_part_two: bool) -> u64 {
    let ranges = parse_ranges(ranges);
    let mut invalid_id_sum = 0;
    for range in ranges {
        for id in range {
            let is_invalid = if !is_part_two {
                is_invalid_id(&id.to_string())
            } else {
                is_invalid_id_part_two(&id.to_string())
            };

            if is_invalid {
                invalid_id_sum += id
            }
        }
    }
    
    invalid_id_sum
}

fn main(){
    let first_result = run(include_str!("puzzle.txt"), false);
    println!("first: {first_result}");

    let second_result = run(include_str!("puzzle.txt"), true);
    println!("second: {second_result}");
}