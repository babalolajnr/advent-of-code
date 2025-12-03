pub fn solve_part1(input: &str) -> isize {
    0
}

pub fn solve_part2(input: &str) -> isize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_solve_part1() {
        let input = std::io::read_to_string(
            File::open("../inputs/day2_input.txt").expect("Failed to open input file"),
        )
        .expect("Failed to read input file");
        assert_eq!(solve_part1(&input), 0);
    }

    #[test]
    fn test_solve_part2() {
        let input = std::io::read_to_string(
            File::open("../inputs/day2_input.txt").expect("Failed to open input file"),
        )
        .expect("Failed to read input file");
        assert_eq!(solve_part2(&input), 0);
    }
}
