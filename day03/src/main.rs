use std::collections::HashSet;
use std::fs;

fn parse_direction(direction: char) -> (i32, i32) {
    match direction {
        '>' => (1, 0),
        '<' => (-1, 0),
        'v' => (0, 1),
        '^' => (0, -1),
        _ => panic!("unknown direction :("),
    }
}

fn part_1(input: &str) -> usize {
    let mut santa_position = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(santa_position);

    input.chars().for_each(|c| {
        let direction = parse_direction(c);
        santa_position.0 += direction.0;
        santa_position.1 += direction.1;
        visited.insert(santa_position);
    });
    visited.len()
}

fn part_2(input: &str) -> usize {
    let mut santa_position = (0, 0);
    let mut robosanta_position = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(santa_position);

    input.chars().enumerate().for_each(|(index, c)| {
        let direction = parse_direction(c);
        if index % 2 == 0 {
            santa_position.0 += direction.0;
            santa_position.1 += direction.1;
            visited.insert(santa_position);
        } else {
            robosanta_position.0 += direction.0;
            robosanta_position.1 += direction.1;
            visited.insert(robosanta_position);
        }
    });
    visited.len()
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();

    assert_eq!(2081, part_1(&content));
    assert_eq!(2341, part_2(&content));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(2, part_1(">"));
        assert_eq!(4, part_1("^>v<"));
        assert_eq!(2, part_1("^v^v^v^v^v"));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(3, part_2("^v"));
        assert_eq!(3, part_2("^>v<"));
        assert_eq!(11, part_2("^v^v^v^v^v"));
    }
}
