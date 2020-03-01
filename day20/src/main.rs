fn part_1(presents: usize) -> usize {
    let div = presents / 10; // optimization
    let mut houses = vec![0; div];

    for elf in 0..div {
        (elf..div).step_by(elf + 1).for_each(|house_number| {
            houses[house_number] += elf + 1;
        });
    }

    houses.iter().position(|&p| p >= div).unwrap() + 1
}

fn part_2(presents: usize) -> usize {
    let mut houses = vec![0; presents];

    for elf in 0..presents {
        (elf..presents)
            .step_by(elf + 1)
            .take(50)
            .for_each(|house_number| {
                houses[house_number] += (elf + 1) * 11;
            });
    }

    houses.iter().position(|&p| p >= presents).unwrap() + 1
}

fn main() {
    const INPUT: usize = 33_100_000;
    assert_eq!(776_160, part_1(INPUT));
    assert_eq!(786_240, part_2(INPUT));
}
