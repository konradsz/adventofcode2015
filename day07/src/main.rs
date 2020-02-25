#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::fs;

type Wires = HashMap<String, u16>;

fn get_value(wires: &Wires, capture: &str) -> Option<u16> {
    capture
        .parse::<u16>()
        .ok()
        .or_else(|| wires.get(&capture.to_string()).cloned())
}

fn emulate_circuit(input: &str, wires: &mut Wires) -> u16 {
    while wires.get("a").is_none() {
        cycle(&input, wires);
    }
    *wires.get("a").unwrap()
}

fn cycle(input: &str, wires: &mut Wires) {
    lazy_static! {
        static ref ASSIGN_RE: Regex = Regex::new(r"^(?P<src>\S+) -> (?P<dst>\S+)$").unwrap();
        static ref AND_RE: Regex =
            Regex::new(r"^(?P<lhs>\S+) AND (?P<rhs>\S+) -> (?P<dst>\S+)$").unwrap();
        static ref OR_RE: Regex =
            Regex::new(r"^(?P<lhs>\S+) OR (?P<rhs>\S+) -> (?P<dst>\S+)$").unwrap();
        static ref LSHIFT_RE: Regex =
            Regex::new(r"^(?P<src>\S+) LSHIFT (?P<shift>\d+) -> (?P<dst>\S+)$").unwrap();
        static ref RSHIFT_RE: Regex =
            Regex::new(r"^(?P<src>\S+) RSHIFT (?P<shift>\d+) -> (?P<dst>\S+)$").unwrap();
        static ref NOT_RE: Regex = Regex::new(r"^NOT (?P<src>\S+) -> (?P<dst>\S+)$").unwrap();
    }

    for instruction in input.lines() {
        if ASSIGN_RE.is_match(instruction) {
            let caps = ASSIGN_RE.captures(instruction).unwrap();
            if let Some(src) = get_value(&wires, &caps["src"]) {
                let dst = caps["dst"].to_string();
                wires.entry(dst).or_insert(src);
            }
        } else if AND_RE.is_match(instruction) {
            let caps = AND_RE.captures(instruction).unwrap();
            if let (Some(lhs), Some(rhs)) = (
                get_value(&wires, &caps["lhs"]),
                get_value(&wires, &caps["rhs"]),
            ) {
                let and = lhs & rhs;
                let dst = caps["dst"].to_string();
                wires.insert(dst, and);
            }
        } else if OR_RE.is_match(instruction) {
            let caps = OR_RE.captures(instruction).unwrap();
            if let (Some(lhs), Some(rhs)) = (
                get_value(&wires, &caps["lhs"]),
                get_value(&wires, &caps["rhs"]),
            ) {
                let or = lhs | rhs;
                let dst = caps["dst"].to_string();
                wires.insert(dst, or);
            }
        } else if LSHIFT_RE.is_match(instruction) {
            let caps = LSHIFT_RE.captures(instruction).unwrap();
            if let Some(src) = get_value(&wires, &caps["src"]) {
                let shift = caps["shift"].parse::<u16>().unwrap();
                let lshift = src << shift;
                let dst = caps["dst"].to_string();
                wires.insert(dst, lshift);
            }
        } else if RSHIFT_RE.is_match(instruction) {
            let caps = RSHIFT_RE.captures(instruction).unwrap();
            if let Some(src) = get_value(&wires, &caps["src"]) {
                let shift = caps["shift"].parse::<u16>().unwrap();
                let rshift = src >> shift;
                let dst = caps["dst"].to_string();
                wires.insert(dst, rshift);
            }
        } else if NOT_RE.is_match(instruction) {
            let caps = NOT_RE.captures(instruction).unwrap();
            if let Some(src) = get_value(&wires, &caps["src"]) {
                let not = !src;
                let dst = caps["dst"].to_string();
                wires.insert(dst, not);
            }
        }
    }
}

fn part_1(input: &str) -> u16 {
    let mut wires: Wires = Wires::new();
    emulate_circuit(&input, &mut wires)
}

fn part_2(input: &str) -> u16 {
    let mut wires: Wires = Wires::new();
    wires.insert(String::from("b"), 3176);
    emulate_circuit(&input, &mut wires)
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();

    assert_eq!(3_176, part_1(input));
    assert_eq!(14_710, part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle() {
        let input = "123 -> x\n\
            456 -> y\n\
            x AND y -> d\n\
            x OR y -> e\n\
            x LSHIFT 2 -> f\n\
            y RSHIFT 2 -> g\n\
            NOT x -> h\n\
            NOT y -> i\n";

        let mut wires: Wires = Wires::new();
        cycle(&input, &mut wires);

        assert_eq!(72, *wires.get("d").unwrap());
        assert_eq!(507, *wires.get("e").unwrap());
        assert_eq!(492, *wires.get("f").unwrap());
        assert_eq!(114, *wires.get("g").unwrap());
        assert_eq!(65412, *wires.get("h").unwrap());
        assert_eq!(65079, *wires.get("i").unwrap());
        assert_eq!(123, *wires.get("x").unwrap());
        assert_eq!(456, *wires.get("y").unwrap());
    }
}
