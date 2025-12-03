fn turn(position: isize, amount: isize) -> isize {
    // Calculate new position on circular track (0-99)
    let position = (position + amount) % 100;

    // Handle negative positions by wrapping around to positive
    if position < 0 {
        return position + 100;
    }

    position
}

fn parse_amount(input: &str) -> isize {
    // Extract first byte to determine direction (R or L)
    let bytes = input.as_bytes();
    let direction = bytes[0];
    // Remaining characters are the numeric amount
    let amount = &input[1..];

    // R = positive (clockwise), L = negative (counter-clockwise)
    if direction == b'R' {
        amount.parse().expect("Failed to parse amount")
    } else if direction == b'L' {
        -amount.parse::<isize>().expect("Failed to parse amount")
    } else {
        panic!("Invalid direction");
    }
}

fn count_zero_crossings(position: isize, amount: isize) -> isize {
    if amount == 0 {
        return 0;
    }

    let abs_amount = amount.abs();
    let new_position = turn(position, amount);

    // Count complete laps
    let mut crossings = abs_amount / 100;

    // For the remaining partial rotation, check if we cross or land on 0
    if amount > 0 {
        // Clockwise: cross 0 if we wrap around (and didn't start at 0)
        if new_position < position && position != 0 {
            crossings += 1;
        } else if new_position == 0 {
            // Landing at 0 always counts
            crossings += 1;
        }
    } else {
        // Counter-clockwise: cross 0 if we wrap around (and didn't start at 0)
        if new_position > position && position != 0 {
            crossings += 1;
        } else if new_position == 0 {
            // Landing at 0 always counts
            crossings += 1;
        }
    }

    crossings
}

pub fn solve_part1(input: &str, start_position: isize) -> isize {
    let mut position: isize = start_position;
    let mut zero_count: isize = 0;

    // Process each turn instruction
    for line in input.lines() {
        let amount = parse_amount(line);
        position = turn(position, amount);
        // Count only when we land exactly on position 0
        if position == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

pub fn solve_part2(input: &str, start_position: isize) -> isize {
    let mut position: isize = start_position;
    let mut zero_count: isize = 0;

    // Process each turn instruction
    for line in input.lines() {
        let amount = parse_amount(line);
        // Count all crossings of position 0 during this move
        zero_count += count_zero_crossings(position, amount);
        position = turn(position, amount);
    }

    zero_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_turn() {
        assert_eq!(turn(10, 5), 15);
        assert_eq!(turn(98, 5), 3);
        assert_eq!(turn(1, -2), 99);
    }

    #[test]
    fn test_parse_amount() {
        assert_eq!(parse_amount("R5"), 5);
        assert_eq!(parse_amount("L3"), -3);
    }

    #[test]
    fn test_solve_part1() {
        let input = std::io::read_to_string(
            File::open("../inputs/day1_input.txt").expect("Failed to open input file"),
        )
        .expect("Failed to read input file");
        assert_eq!(solve_part1(&input, 50), 1034);
    }

    #[test]
    fn test_count_zero_crossings() {
        assert_eq!(count_zero_crossings(50, 68), 1);
        assert_eq!(count_zero_crossings(50, -68), 1);
        assert_eq!(count_zero_crossings(52, 48), 1);
        assert_eq!(count_zero_crossings(95, 60), 1);
        assert_eq!(count_zero_crossings(55, -55), 1);
        assert_eq!(count_zero_crossings(99, -99), 1);
        assert_eq!(count_zero_crossings(14, -82), 1);
        assert_eq!(count_zero_crossings(50, 1000), 10);
        assert_eq!(count_zero_crossings(82, -30), 0);
        assert_eq!(count_zero_crossings(0, -5), 0);
        assert_eq!(count_zero_crossings(0, -1), 0);
        assert_eq!(count_zero_crossings(0, 14), 0);
    }

    #[test]
    fn test_solve_part2_sample() {
        let input = std::io::read_to_string(
            File::open("../inputs/day1_sample.txt").expect("Failed to open input file"),
        )
        .expect("Failed to read input file");
        assert_eq!(solve_part2(&input, 50), 6);
    }

    #[test]
    fn test_solve_part2() {
        let input = std::io::read_to_string(
            File::open("../inputs/day1_input.txt").expect("Failed to open input file"),
        )
        .expect("Failed to read input file");
        assert_eq!(solve_part2(&input, 50), 6166);
    }
}
