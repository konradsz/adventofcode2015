use std::fs;

fn get_dimensions(input: &str) -> Vec<u32> {
    let mut dimensions: Vec<u32> = input
        .split('x')
        .map(|dimension| dimension.parse().unwrap())
        .collect();
    dimensions.sort();
    dimensions
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let dim = get_dimensions(line);
            let (a, b, c) = (dim[0], dim[1], dim[2]);
            let sides = vec![a * b, a * c, b * c];
            sides.iter().map(|side| 2 * side).sum::<u32>() + a * b
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let dim = get_dimensions(line);
            let (a, b, c) = (dim[0], dim[1], dim[2]);
            2 * a + 2 * b + a * b * c
        })
        .sum()
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();

    assert_eq!(1606483, part_1(&content));
    assert_eq!(3842356, part_2(&content));
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