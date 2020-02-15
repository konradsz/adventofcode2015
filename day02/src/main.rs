use std::fs;

fn get_dimensions(input: &str) -> (u32, u32, u32) {
    let dimensions: Vec<u32> = input
        .split('x')
        .map(|dimension| dimension.parse().unwrap())
        .collect();
    (dimensions[0], dimensions[1], dimensions[2])
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (l, w, h) = get_dimensions(line);
            let sides = vec![l * w, l * h, w * h];
            let min_side = sides.iter().min().unwrap();
            sides.iter().map(|side| 2 * side).sum::<u32>() + min_side
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (l, w, h) = get_dimensions(line);
            vec![2 * l + 2 * w, 2 * l + 2 * h, 2 * w + 2 * h]
                .iter()
                .min()
                .unwrap()
                + l * w * h
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
