use regex::Regex;
use std::fs;

const GRID_SIZE: usize = 1_000;

struct Rectangle {
    top_x: usize,
    top_y: usize,
    bottom_x: usize,
    bottom_y: usize,
}

fn parse_line(line: &str, regex: &Regex) -> Rectangle {
    let caps = regex.captures(line).unwrap();
    Rectangle {
        top_x: caps[1].parse::<usize>().unwrap(),
        top_y: caps[2].parse::<usize>().unwrap(),
        bottom_x: caps[3].parse::<usize>().unwrap(),
        bottom_y: caps[4].parse::<usize>().unwrap(),
    }
}

fn apply<Command>(grid: &mut Vec<u32>, rectangle: &Rectangle, command: Command)
where
    Command: Fn(&mut u32),
{
    for row in rectangle.top_y..=rectangle.bottom_y {
        for column in rectangle.top_x..=rectangle.bottom_x {
            command(&mut grid[row * GRID_SIZE + column]);
        }
    }
}

fn part_1(input: &str) -> usize {
    let mut grid = vec![0; GRID_SIZE * GRID_SIZE];

    let turn_on_light = |light: &mut u32| *light = 1;
    let turn_off_light = |light: &mut u32| *light = 0;
    let toggle_light = |light: &mut u32| {
        if *light == 0 {
            *light = 1;
        } else if *light == 1 {
            *light = 0;
        }
    };

    let turn_on_regex = Regex::new(r"^turn on (\d+),(\d+) through (\d+),(\d+)$").unwrap();
    let turn_off_regex = Regex::new(r"^turn off (\d+),(\d+) through (\d+),(\d+)$").unwrap();
    let toggle_regex = Regex::new(r"^toggle (\d+),(\d+) through (\d+),(\d+)$").unwrap();

    for line in input.lines() {
        if turn_on_regex.is_match(&line) {
            let rectangle = parse_line(line, &turn_on_regex);
            apply(&mut grid, &rectangle, turn_on_light);
        } else if turn_off_regex.is_match(&line) {
            let rectangle = parse_line(line, &turn_off_regex);
            apply(&mut grid, &rectangle, turn_off_light);
        } else if toggle_regex.is_match(&line) {
            let rectangle = parse_line(line, &toggle_regex);
            apply(&mut grid, &rectangle, toggle_light);
        }
    }

    grid.iter().filter(|&light| *light == 1).count()
}

fn part_2(input: &str) -> u32 {
    let mut grid = vec![0; GRID_SIZE * GRID_SIZE];

    let turn_on_light = |light: &mut u32| *light += 1;
    let turn_off_light = |light: &mut u32| *light = light.saturating_sub(1);
    let toggle_light = |light: &mut u32| *light += 2;

    let turn_on_regex = Regex::new(r"^turn on (\d+),(\d+) through (\d+),(\d+)$").unwrap();
    let turn_off_regex = Regex::new(r"^turn off (\d+),(\d+) through (\d+),(\d+)$").unwrap();
    let toggle_regex = Regex::new(r"^toggle (\d+),(\d+) through (\d+),(\d+)$").unwrap();

    for line in input.lines() {
        if turn_on_regex.is_match(&line) {
            let rectangle = parse_line(line, &turn_on_regex);
            apply(&mut grid, &rectangle, turn_on_light);
        } else if turn_off_regex.is_match(&line) {
            let rectangle = parse_line(line, &turn_off_regex);
            apply(&mut grid, &rectangle, turn_off_light);
        } else if toggle_regex.is_match(&line) {
            let rectangle = parse_line(line, &toggle_regex);
            apply(&mut grid, &rectangle, toggle_light);
        }
    }

    grid.iter().sum()
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();

    assert_eq!(377891, part_1(content));
    assert_eq!(14110788, part_2(content));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(GRID_SIZE * GRID_SIZE, part_1("turn on 0,0 through 999,999"));
        assert_eq!(GRID_SIZE, part_1("toggle 0,0 through 999,0"));
        assert_eq!(4, part_1("turn on 499,499 through 500,500"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1, part_2("turn on 0,0 through 0,0"));
        assert_eq!((2 * GRID_SIZE * GRID_SIZE) as u32, part_2("toggle 0,0 through 999,999"));
    }
}
