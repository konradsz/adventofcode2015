use itertools::Itertools;

fn process(vec: &mut Vec<usize>) {
    *vec = vec
        .iter()
        .group_by(|&x| x)
        .into_iter()
        .fold(Vec::new(), |mut output, (key, group)| {
            output.push(group.count());
            output.push(*key);
            output
        });
}

fn part_1() -> usize {
    let mut vec = vec![1, 3, 2, 1, 1, 3, 1, 1, 1, 2];
    (0..40).for_each(|_| process(&mut vec));
    vec.len()
}

fn part_2() -> usize {
    let mut vec = vec![1, 3, 2, 1, 1, 3, 1, 1, 1, 2];
    (0..50).for_each(|_| process(&mut vec));
    vec.len()
}

fn main() {
    assert_eq!(492982, part_1());
    assert_eq!(6989950, part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(process(&vec![1]), vec![1, 1]);
        assert_eq!(process(&vec![1, 1]), vec![2, 1]);
        assert_eq!(process(&vec![2, 1]), vec![1, 2, 1, 1]);
        assert_eq!(process(&vec![1, 2, 1, 1]), vec![1, 1, 1, 2, 2, 1]);
        assert_eq!(process(&vec![1, 1, 1, 2, 2, 1]), vec![3, 1, 2, 2, 1, 1]);
    }
}
