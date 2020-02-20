use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn get_value(wires: &HashMap<String, u16>, capture: &str) -> Option<u16> {
    if let Some(number) = capture.parse::<u16>().ok() {
        return Some(number);
    }
    return wires.get(&capture.to_string()).cloned();
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();

    let assign_regex = Regex::new(r"^(?P<src>\S+) -> (?P<dst>\S+)$").unwrap();
    let and_regex =
        Regex::new(r"^(?P<lhs>\S+) AND (?P<rhs>\S+) -> (?P<dst>\S+)$").unwrap();
    let or_regex =
        Regex::new(r"^(?P<lhs>\S+) OR (?P<rhs>\S+) -> (?P<dst>\S+)$").unwrap();
    let lshift_regex =
        Regex::new(r"^(?P<src>\S+) LSHIFT (?P<shift>\d+) -> (?P<dst>\S+)$").unwrap();
    let rshift_regex =
        Regex::new(r"^(?P<src>\S+) RSHIFT (?P<shift>\d+) -> (?P<dst>\S+)$").unwrap();
    let not_regex = Regex::new(r"^NOT (?P<src>\S+) -> (?P<dst>\S+)$").unwrap();

    let mut wires = HashMap::new();

    // PART 2
    wires.insert(String::from("b"), 3176);

    while wires.get("a").is_none() {
        for instruction in content.lines() {
            if assign_regex.is_match(instruction) {
                let caps = assign_regex.captures(instruction).unwrap();
                if let Some(src) = get_value(&wires, &caps["src"]) {
                    let dst = caps["dst"].to_string();
                    wires.entry(dst).or_insert(src);
                }
            } else if and_regex.is_match(instruction) {
                let caps = and_regex.captures(instruction).unwrap();
                let lhs = get_value(&wires, &caps["lhs"]);
                let rhs = get_value(&wires, &caps["rhs"]);
                if lhs.is_some() && rhs.is_some() {
                    let and = lhs.unwrap() & rhs.unwrap();
                    let dst = caps["dst"].to_string();
                    wires.insert(dst, and);
                }
            } else if or_regex.is_match(instruction) {
                let caps = or_regex.captures(instruction).unwrap();
                let lhs = get_value(&wires, &caps["lhs"]);
                let rhs = get_value(&wires, &caps["rhs"]);
                if lhs.is_some() && rhs.is_some() {
                    let or = lhs.unwrap() | rhs.unwrap();
                    let dst = caps["dst"].to_string();
                    wires.insert(dst, or);
                }
            } else if lshift_regex.is_match(instruction) {
                let caps = lshift_regex.captures(instruction).unwrap();
                if let Some(src) = get_value(&wires, &caps["src"]) {
                    let shift = caps["shift"].parse::<u16>().unwrap();
                    let lshift = src << shift;
                    let dst = caps["dst"].to_string();
                    wires.insert(dst, lshift);
                }
            } else if rshift_regex.is_match(instruction) {
                let caps = rshift_regex.captures(instruction).unwrap();
                if let Some(src) = get_value(&wires, &caps["src"]) {
                    let shift = caps["shift"].parse::<u16>().unwrap();
                    let rshift = src >> shift;
                    let dst = caps["dst"].to_string();
                    wires.insert(dst, rshift);
                }
            } else if not_regex.is_match(instruction) {
                let caps = not_regex.captures(instruction).unwrap();
                if let Some(src) = get_value(&wires, &caps["src"]) {
                    let not = !src;
                    let dst = caps["dst"].to_string();
                    wires.insert(dst, not);
                }
            }
        }
    }

    println!("{}", wires.get("a").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
    }
}
