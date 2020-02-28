use std::fs;

type LightGrid = Vec<Vec<Light>>;

const GRID_SIZE: usize = 100;

#[derive(Clone, PartialEq)]
enum Light {
    On,
    Off,
}

fn part_1(mut grid: LightGrid) -> usize {
    const STEPS: usize = 100;
    (0..STEPS).for_each(|_| {
        grid = process_grid(&grid);
    });

    count_turned_on_lights(&grid)
}

fn part_2(mut grid: LightGrid) -> usize {
    const STEPS: usize = 100;
    (0..STEPS).for_each(|_| {
        grid = process_grid(&grid);
        grid[0][0] = Light::On;
        grid[0][GRID_SIZE - 1] = Light::On;
        grid[GRID_SIZE - 1][0] = Light::On;
        grid[GRID_SIZE - 1][GRID_SIZE - 1] = Light::On;
    });

    count_turned_on_lights(&grid)
}

fn process_grid(grid: &LightGrid) -> LightGrid {
    let mut processed_grid = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        let mut new_row = Vec::new();
        for (x, light) in row.iter().enumerate() {
            let neighbours = count_neighbours(x, y, &grid);

            match *light {
                Light::On => {
                    if neighbours == 2 || neighbours == 3 {
                        new_row.push(Light::On);
                    } else {
                        new_row.push(Light::Off);
                    }
                }
                Light::Off => {
                    if neighbours == 3 {
                        new_row.push(Light::On);
                    } else {
                        new_row.push(Light::Off);
                    }
                }
            }
        }

        processed_grid.push(new_row);
    }

    processed_grid
}

fn has_neighbour(x: usize, y: usize, offset: (i32, i32)) -> bool {
    (offset.0 >= 0 || x != 0)
        && (offset.0 <= 0 || x != GRID_SIZE - 1)
        && (offset.1 >= 0 || y != 0)
        && (offset.1 <= 0 || y != GRID_SIZE - 1)
}

fn count_neighbours(x: usize, y: usize, grid: &LightGrid) -> usize {
    [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ]
    .iter()
    .filter(|offset| has_neighbour(x, y, **offset))
    .filter(|offset| {
        grid[(y as i32 + offset.1) as usize][(x as i32 + offset.0) as usize] == Light::On
    })
    .count()
}

fn count_turned_on_lights(grid: &LightGrid) -> usize {
    grid.iter()
        .flatten()
        .filter(|light| **light == Light::On)
        .count()
}

fn parse_input(input: &str) -> LightGrid {
    let mut grid: LightGrid = Vec::new();

    for line in input.lines() {
        let row = line
            .chars()
            .map(|c| match c {
                '#' => Light::On,
                '.' => Light::Off,
                _ => panic!("no no no"),
            })
            .collect();

        grid.push(row);
    }

    grid
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();

    let grid = parse_input(input);

    assert_eq!(768, part_1(grid.clone()));
    assert_eq!(781, part_2(grid));
}
