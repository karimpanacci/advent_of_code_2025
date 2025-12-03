use std::ops::{Range};
use std::str::FromStr;
use advent_of_code::utils::overflow_with_range;

/// The bool inside tuple return true if turn is R, false if is L
fn parse_turn(turn: &str) -> Option<(bool, i32)> {
    let mut turn_chars = turn.chars();
    let rotation = turn_chars.next();
    let value: String = turn_chars.collect();
    let value = i32::from_str(&value);
    let (Some(rotation), Ok(value)) = (rotation,value) else {
        return None;
    };

    let rotation = match rotation {
        'R' => true,
        'L' => false,
        _ => return None,
    };

    Some((rotation,value))
}

fn run(start_value: i32, turns: &str, range: Range<i32>) -> u32 {
    let turns = turns.split("\n");

    let mut current_value = start_value;
    let mut zero_count: u32 = 0;

    for turn in turns {
        let Some((rotation, value)) = parse_turn(turn) else {
            continue
        };

        println!("current_value = {current_value}");
        if rotation {
            print!("{current_value} + {value} ");
            current_value += value
        } else {
            print!("{current_value} - {value} ");
            current_value -= value
        }


        current_value = overflow_with_range(&range, current_value);
        println!("= {current_value}");

        if current_value == 0 {
            zero_count += 1;
            println!("ZERO COUNT {zero_count}");
        }
    };

    zero_count
}


fn main() {
    let password = run(50, include_str!("puzzle.txt"), 0..100);
    println!("{password}");
}

#[cfg(test)]
mod test {
    use crate::{overflow_with_range, parse_turn, run};

    #[test]
    fn overflow_test(){
        assert_eq!(overflow_with_range(&(0..100), 0), 0);
        assert_eq!(overflow_with_range(&(0..100), -1), 99);
        assert_eq!(overflow_with_range(&(0..100), -2), 98);
        assert_eq!(overflow_with_range(&(0..100), -101), 99);
        assert_eq!(overflow_with_range(&(0..100), 99), 99);
        assert_eq!(overflow_with_range(&(0..100), 100), 0);
        assert_eq!(overflow_with_range(&(0..100), 101), 1);
        assert_eq!(overflow_with_range(&(0..100), 201), 1);
    }

    #[test]
    fn parse_turn_test(){
        assert_eq!(parse_turn("L13"), Some((false, 13)));
        assert_eq!(parse_turn("R17"), Some((true, 17)));
        assert_eq!(parse_turn("L"), None);
        assert_eq!(parse_turn("17"), None);
        assert_eq!(parse_turn("L 17"), None);
    }

    #[test]
    fn main_test(){
        let turns = "\
        L10\n\
        R5\n\
        R5\n\
        R5\n\
        L4\n\
        L1\n\
        ";
        assert_eq!(run(0, turns, 0..100), 2);
    }
}