use permutohedron::LexicalPermutation;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

type RelationshipList = HashMap<String, Vec<Relationship>>;

struct Relationship {
    name: String,
    gain: i32,
}

impl Relationship {
    fn new(name: String, gain: i32) -> Self {
        Relationship { name, gain }
    }
}

fn part_1(input: &str) -> i32 {
    let list = parse_list(&input);
    calculate_happiness_change(&list)
}

fn part_2(input: &str) -> i32 {
    let list = parse_list(&input);
    let list = add_me_to_list(list);
    calculate_happiness_change(&list)
}

fn parse_list(input: &str) -> RelationshipList {
    let mut list: RelationshipList = RelationshipList::new();

    let entry_regex = Regex::new(
        r"^(?P<lhs>\S+) would (?P<impact>\S+) (?P<gain>\d+) happiness units by sitting next to (?P<rhs>\S+).$"
    ).unwrap();

    for line in input.lines() {
        let caps = entry_regex.captures(line).unwrap();
        let entry = list.entry(String::from(&caps["lhs"])).or_default();
        let gain = match &caps["impact"] {
            "gain" => caps["gain"].parse::<i32>().unwrap(),
            "lose" => -caps["gain"].parse::<i32>().unwrap(),
            _ => unreachable!(),
        };
        (*entry).push(Relationship::new(String::from(&caps["rhs"]), gain));
    }

    list
}

fn add_me_to_list(mut list: RelationshipList) -> RelationshipList {
    let names: Vec<String> = list.keys().cloned().collect();
    list.values_mut()
        .for_each(|value| value.push(Relationship::new(String::from("Me"), 0)));
    let my_relationships: Vec<Relationship> = names
        .into_iter()
        .map(|name| Relationship::new(name, 0))
        .collect();
    list.insert(String::from("Me"), my_relationships);

    list
}

fn calculate_happiness_change(list: &RelationshipList) -> i32 {
    let mut names: Vec<String> = list.keys().cloned().collect();

    let mut optimal_happiness_change = 0;
    loop {
        let mut current_happiness_change = 0;
        let all_names = &names[0..];
        let first_name = &names[0..1];

        for pair in [all_names, first_name].concat().windows(2) {
            let lhs = &pair[0];
            let rhs = &pair[1];

            let lhs_entries = list.get(lhs.as_str()).unwrap();
            for entry in lhs_entries {
                if entry.name == *rhs {
                    current_happiness_change += entry.gain;
                }
            }

            let rhs_entries = list.get(rhs.as_str()).unwrap();
            for entry in rhs_entries {
                if entry.name == *lhs {
                    current_happiness_change += entry.gain;
                }
            }
        }

        if current_happiness_change > optimal_happiness_change {
            optimal_happiness_change = current_happiness_change;
        }

        if !names.next_permutation() {
            break;
        }
    }

    optimal_happiness_change
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();

    assert_eq!(709, part_1(&input));
    assert_eq!(668, part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "Alice would gain 54 happiness units by sitting next to Bob.\n\
                     Alice would lose 79 happiness units by sitting next to Carol.\n\
                     Alice would lose 2 happiness units by sitting next to David.\n\
                     Bob would gain 83 happiness units by sitting next to Alice.\n\
                     Bob would lose 7 happiness units by sitting next to Carol.\n\
                     Bob would lose 63 happiness units by sitting next to David.\n\
                     Carol would lose 62 happiness units by sitting next to Alice.\n\
                     Carol would gain 60 happiness units by sitting next to Bob.\n\
                     Carol would gain 55 happiness units by sitting next to David.\n\
                     David would gain 46 happiness units by sitting next to Alice.\n\
                     David would lose 7 happiness units by sitting next to Bob.\n\
                     David would gain 41 happiness units by sitting next to Carol.\n";

        assert_eq!(330, part_1(&input));
    }
}
