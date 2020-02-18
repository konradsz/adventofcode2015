use std::fs;

fn has_at_least_three_vowels(input: &str) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    input
        .chars()
        .fold(0, |acc, c| acc + vowels.contains(&c) as u32)
        >= 3
}

fn has_letter_that_appears_twice_in_a_row(input: &str) -> bool {
    input
        .as_bytes()
        .windows(2)
        .any(|window| window[0] == window[1])
}

fn does_not_contain_excluded_strings(input: &str) -> bool {
    let excluded = ["ab", "cd", "pq", "xy"];
    !excluded.iter().any(|s| input.contains(s))
}

fn has_two_letters_that_appears_twice_without_overlapping(input: &str) -> bool {
    input
        .as_bytes()
        .windows(2)
        .enumerate()
        .any(|(index, window)| {
            input
                .as_bytes()
                .windows(2)
                .skip(index + 2)
                .any(|second| window[0] == second[0] && window[1] == second[1])
        })
}

fn has_letter_which_repeats_with_exactly_one_letter_between(input: &str) -> bool {
    input
        .as_bytes()
        .windows(3)
        .any(|window| window[0] == window[2])
}

fn is_string_nice_part_1(input: &str) -> bool {
    has_at_least_three_vowels(input)
        && has_letter_that_appears_twice_in_a_row(input)
        && does_not_contain_excluded_strings(input)
}

fn is_string_nice_part_2(input: &str) -> bool {
    has_two_letters_that_appears_twice_without_overlapping(input)
        && has_letter_which_repeats_with_exactly_one_letter_between(input)
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| is_string_nice_part_1(line))
        .count()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| is_string_nice_part_2(line))
        .count()
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();

    assert_eq!(236, part_1(&content));
    assert_eq!(51, part_2(&content));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(is_string_nice_part_1("ugknbfddgicrmopn"));
        assert!(is_string_nice_part_1("aaa"));
        assert!(!is_string_nice_part_1("jchzalrnumimnmhp"));
        assert!(!is_string_nice_part_1("haegwjzuvuyypxyu"));
        assert!(!is_string_nice_part_1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_part2() {
        assert!(is_string_nice_part_2("qjhvhtzxzqqjkmpb"));
        assert!(is_string_nice_part_2("xxyxx"));
        assert!(!is_string_nice_part_2("uurcxstgmygtbstg"));
        assert!(!is_string_nice_part_2("ieodomkazucvgmuy"));
    }
}
