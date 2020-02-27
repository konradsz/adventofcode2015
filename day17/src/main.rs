use std::collections::HashMap;

const LITERS: usize = 150;

fn take_next_container(
    containers: &[usize],
    combinations: &mut HashMap<usize, usize>,
    current_volume: usize,
    containers_used: usize,
) {
    for (index, _) in containers.iter().enumerate() {
        let next_volume = current_volume + containers[index];
        if next_volume == LITERS {
            let combination = combinations.entry(containers_used).or_insert(0);
            *combination += 1;
            continue;
        } else if next_volume < LITERS {
            take_next_container(
                &containers[index + 1..],
                combinations,
                next_volume,
                containers_used + 1,
            );
        }
    }
}

fn main() {
    let containers = vec![
        33, 14, 18, 20, 45, 35, 16, 35, 1, 13, 18, 13, 50, 44, 48, 6, 24, 41, 30, 42,
    ];

    let mut combinations = HashMap::new();
    take_next_container(&containers, &mut combinations, 0, 0);

    assert_eq!(1304, combinations.values().sum::<usize>());
    assert_eq!(18, *combinations.iter().min().unwrap().1);
}
