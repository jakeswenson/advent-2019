use std::borrow::Borrow;
use std::ops::{Range, RangeInclusive};

fn get_digits(mut num: i32, buffer: &mut Vec<i32>) {
    buffer.clear();
    while num > 0 {
        buffer.push(num % 10);
        num /= 10;
    }

    buffer.reverse();
}

fn meets_rules_part1(num: i32, buffer: &mut Vec<i32>) -> bool {
    get_digits(num, buffer);

    struct State(bool, i32, bool);

    let state = buffer.iter().fold(State(true, 0, false), |state, &digit| {
        let is_increasing = digit > state.1;
        let is_equal = digit == state.1;
        State(
            state.0 && (is_increasing || is_equal),
            digit,
            state.2 || is_equal,
        )
    });

    state.0 && state.2
}

fn part1(range: RangeInclusive<i32>) -> usize {
    let mut buffer = Vec::with_capacity(6);
    range
        .filter(|&num| meets_rules_part1(num, &mut buffer))
        .count()
}

fn meets_rules_part2(num: i32, buffer: &mut Vec<i32>) -> bool {
    get_digits(num, buffer);

    struct State {
        increasing: bool,
        last: Option<i32>,
        last_last: Option<i32>,
        has_valid_double: bool,
        valid_double_digit: Option<i32>,
    };

    let state = buffer.iter().fold(
        State {
            increasing: true,
            has_valid_double: false,
            last: None,
            last_last: None,
            valid_double_digit: None,
        },
        |state, &digit| {
            let is_increasing_or_equal =
                state.last.map(|last_dig| digit >= last_dig).unwrap_or(true);
            let is_valid_double = Some(digit) == state.last && Some(digit) != state.last_last;
            State {
                increasing: state.increasing && is_increasing_or_equal,
                last: Some(digit),
                last_last: state.last,
                has_valid_double: (Some(digit) != state.valid_double_digit
                    && state.has_valid_double)
                    || is_valid_double,
                valid_double_digit: if is_valid_double { Some(digit) } else { None },
            }
        },
    );

    state.increasing && state.has_valid_double
}

fn part2(range: RangeInclusive<i32>) -> usize {
    let mut buffer = Vec::with_capacity(6);
    range
        .filter(|&num| meets_rules_part2(num, &mut buffer))
        .count()
}

pub fn solve() {
    println!("Digits Between: {}", part1(138241..=674034));
    println!("Digits Part 02: {}", part2(138241..=674034));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_digits_fun() {
        let mut buffer = Vec::new();
        get_digits(111123, &mut buffer);
        assert_eq!(buffer, vec![1, 1, 1, 1, 2, 3]);
    }

    #[test]
    fn test_part1_rules() {
        let mut buffer = Vec::new();
        assert!(meets_rules_part1(111111, &mut buffer));
        assert!(!meets_rules_part1(223450, &mut buffer));
        assert!(!meets_rules_part1(123789, &mut buffer))
    }

    #[test]
    fn test_part2_rules() {
        let mut buffer = Vec::new();
        assert!(meets_rules_part2(112233, &mut buffer));
        assert!(!meets_rules_part2(123444, &mut buffer));
        assert!(meets_rules_part2(111122, &mut buffer));
        assert!(meets_rules_part2(123344, &mut buffer));
        //assert!(meets_rules_part2(12334567, &mut buffer));
    }
}
