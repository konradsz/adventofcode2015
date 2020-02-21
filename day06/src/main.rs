#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::fs;

const GRID_SIZE: usize = 1_000;

struct Rectangle {
    top_x: usize,
    top_y: usize,
    bottom_x: usize,
    bottom_y: usize,
}

fn parse_line(line: &str) -> (String, Rectangle) {
    lazy_static! {
        static ref INSTRUCTION: Regex = Regex::new(
            r"^(?P<cmd>.+) (?P<top_x>\d+),(?P<top_y>\d+) through (?P<bottom_x>\d+),(?P<bottom_y>\d+)$",
        )
        .unwrap();
    }
    let caps = INSTRUCTION.captures(line).unwrap();

    (
        String::from(&caps["cmd"]),
        Rectangle {
            top_x: caps["top_x"].parse::<usize>().unwrap(),
            top_y: caps["top_y"].parse::<usize>().unwrap(),
            bottom_x: caps["bottom_x"].parse::<usize>().unwrap(),
            bottom_y: caps["bottom_y"].parse::<usize>().unwrap(),
        },
    )
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

fn process_grid<TurnOnLight, TurnOffLight, ToggleLight>(
    input: &str,
    grid: &mut Vec<u32>,
    turn_on_light: &TurnOnLight,
    turn_off_light: &TurnOffLight,
    toggle_light: &ToggleLight,
) where
    TurnOnLight: Fn(&mut u32),
    TurnOffLight: Fn(&mut u32),
    ToggleLight: Fn(&mut u32),
{
    for line in input.lines() {
        let (command, rectangle) = parse_line(line);
        match command.as_str() {
            "turn on" => apply(grid, &rectangle, turn_on_light),
            "turn off" => apply(grid, &rectangle, turn_off_light),
            "toggle" => apply(grid, &rectangle, toggle_light),
            _ => panic!("unknown command"),
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

    process_grid(
        input,
        &mut grid,
        &turn_on_light,
        &turn_off_light,
        &toggle_light,
    );

    grid.iter().filter(|&light| *light == 1).count()
}

fn part_2(input: &str) -> u32 {
    let mut grid = vec![0; GRID_SIZE * GRID_SIZE];

    let turn_on_light = |light: &mut u32| *light += 1;
    let turn_off_light = |light: &mut u32| *light = light.saturating_sub(1);
    let toggle_light = |light: &mut u32| *light += 2;

    process_grid(
        input,
        &mut grid,
        &turn_on_light,
        &turn_off_light,
        &toggle_light,
    );

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
    fn test_part_1() {
        assert_eq!(GRID_SIZE * GRID_SIZE, part_1("turn on 0,0 through 999,999"));
        assert_eq!(GRID_SIZE, part_1("toggle 0,0 through 999,0"));
        assert_eq!(4, part_1("turn on 499,499 through 500,500"));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(1, part_2("turn on 0,0 through 0,0"));
        assert_eq!(
            (2 * GRID_SIZE * GRID_SIZE) as u32,
            part_2("toggle 0,0 through 999,999")
        );
    }
}
