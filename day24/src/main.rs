use itertools::Itertools;

fn find_first_subgroup_that_sums_to(
    weights: &[usize],
    sum: usize,
) -> Option<std::vec::Vec<&usize>> {
    let mut it = (1..weights.len() - 2)
        .map(|group_size| {
            weights
                .iter()
                .combinations(group_size)
                .find(|group| group.iter().copied().sum::<usize>() == sum)
        })
        .skip_while(|combination| combination.is_none());

    // weights are sorted, so first found combination is "ideal"
    it.next()
        .expect(&format!("there is no subgroup that sums to {}", sum))
}

fn part_1(weights: &[usize]) -> usize {
    let group_sum = weights.iter().sum::<usize>() / 3;

    if let Some(combination) = find_first_subgroup_that_sums_to(weights, group_sum) {
        return combination.into_iter().product();
    }

    unreachable!()
}

fn part_2(weights: &[usize]) -> usize {
    let group_sum = weights.iter().sum::<usize>() / 4;

    if let Some(combination) = find_first_subgroup_that_sums_to(weights, group_sum) {
        return combination.into_iter().product();
    }

    unreachable!()
}

fn main() {
    let weights = [
        1, 2, 3, 5, 7, 13, 17, 19, 23, 29, 31, 37, 41, 43, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
        101, 103, 107, 109, 113,
    ];

    assert_eq!(10_723_906_903, part_1(&weights));
    assert_eq!(74_850_409, part_2(&weights));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let weights = [1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        assert_eq!(99, part_1(&weights));
    }
}
