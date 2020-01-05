use std::fs;

fn step(c: char) -> i32 {
    if c == '(' {
        1
    } else if c == ')' {
        -1
    } else {
        0
    }
}

fn part_1(input: &str) -> i32 {
    input.chars().fold(0, |mut acc, c| {
        acc += step(c);
        acc
    })
}

fn part_2(input: &str) -> usize {
    let mut current_floor = 0;
    input
        .chars()
        .take_while(|c| {
            current_floor += step(*c);
            current_floor != -1
        })
        .count()
        + 1
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();

    assert_eq!(part_1(&content), 74);
    assert_eq!(part_2(&content), 1795);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("(())"), 0);
        assert_eq!(part_1("()()"), 0);
        assert_eq!(part_1("((("), 3);
        assert_eq!(part_1("(()(()("), 3);
        assert_eq!(part_1("))((((("), 3);
        assert_eq!(part_1("())"), -1);
        assert_eq!(part_1("))("), -1);
        assert_eq!(part_1(")))"), -3);
        assert_eq!(part_1(")())())"), -3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(")"), 1);
        assert_eq!(part_2("()())"), 5);
    }
}