use std::fs;

fn get_dimensions(input: &str) -> (u32, u32, u32) {
    let mut dimensions: Vec<u32> = input
        .split('x')
        .map(|dimension| dimension.parse().unwrap())
        .collect();
    dimensions.sort();
    (dimensions[0], dimensions[1], dimensions[2])
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (a, b, c) = get_dimensions(line);
            let sides = vec![a * b, a * c, b * c];
            sides.iter().map(|side| 2 * side).sum::<u32>() + a * b
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (a, b, c) = get_dimensions(line);
            2 * a + 2 * b + a * b * c
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();

    assert_eq!(1_606_483, part_1(input));
    assert_eq!(3_842_356, part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(58, part_1("2x3x4"));
        assert_eq!(43, part_1("1x1x10"));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(34, part_2("2x3x4"));
        assert_eq!(14, part_2("1x1x10"));
    }
}
