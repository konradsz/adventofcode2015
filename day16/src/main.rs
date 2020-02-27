#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::fs;

struct Aunt {
    number: u32,
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

fn parse_line(input: &str) -> Aunt {
    lazy_static! {
        static ref NUMBER_RE: Regex = Regex::new(r"^Sue (?P<number>\d+):.*$").unwrap();
        static ref CHILDREN_RE: Regex = Regex::new(r"^.*children: (?P<children>\d+).*$").unwrap();
        static ref CATS_RE: Regex = Regex::new(r"^.*cats: (?P<cats>\d+).*$").unwrap();
        static ref SAMOYEDS_RE: Regex = Regex::new(r"^.*samoyeds: (?P<samoyeds>\d+).*$").unwrap();
        static ref POMERANIANS_RE: Regex =
            Regex::new(r"^.*pomeranians: (?P<pomeranians>\d+).*$").unwrap();
        static ref AKITAS_RE: Regex = Regex::new(r"^.*akitas: (?P<akitas>\d+).*$").unwrap();
        static ref VIZSLAS_RE: Regex = Regex::new(r"^.*vizslas: (?P<vizslas>\d+).*$").unwrap();
        static ref GOLDFISH_RE: Regex = Regex::new(r"^.*goldfish: (?P<goldfish>\d+).*$").unwrap();
        static ref TREES_RE: Regex = Regex::new(r"^.*trees: (?P<trees>\d+).*$").unwrap();
        static ref CARS_RE: Regex = Regex::new(r"^.*cars: (?P<cars>\d+).*$").unwrap();
        static ref PERFUMES_RE: Regex = Regex::new(r"^.*perfumes: (?P<perfumes>\d+).*$").unwrap();
    }

    Aunt {
        number: NUMBER_RE
            .captures(input)
            .map(|cap| cap["number"].parse::<u32>().unwrap())
            .unwrap(),
        children: CHILDREN_RE
            .captures(input)
            .map(|cap| cap["children"].parse::<u32>().unwrap()),
        cats: CATS_RE
            .captures(input)
            .map(|cap| cap["cats"].parse::<u32>().unwrap()),
        samoyeds: SAMOYEDS_RE
            .captures(input)
            .map(|cap| cap["samoyeds"].parse::<u32>().unwrap()),
        pomeranians: POMERANIANS_RE
            .captures(input)
            .map(|cap| cap["pomeranians"].parse::<u32>().unwrap()),
        akitas: AKITAS_RE
            .captures(input)
            .map(|cap| cap["akitas"].parse::<u32>().unwrap()),
        vizslas: VIZSLAS_RE
            .captures(input)
            .map(|cap| cap["vizslas"].parse::<u32>().unwrap()),
        goldfish: GOLDFISH_RE
            .captures(input)
            .map(|cap| cap["goldfish"].parse::<u32>().unwrap()),
        trees: TREES_RE
            .captures(input)
            .map(|cap| cap["trees"].parse::<u32>().unwrap()),
        cars: CARS_RE
            .captures(input)
            .map(|cap| cap["cars"].parse::<u32>().unwrap()),
        perfumes: PERFUMES_RE
            .captures(input)
            .map(|cap| cap["perfumes"].parse::<u32>().unwrap()),
    }
}

fn part_1(aunts: &[Aunt]) -> u32 {
    let equal_or_none = |field: Option<u32>, expected: u32| -> bool {
        if let Some(count) = field {
            if count != expected {
                return false;
            }
        }
        true
    };

    let real_aunt = aunts
        .iter()
        .find(|aunt| {
            equal_or_none(aunt.children, 3)
                && equal_or_none(aunt.cats, 7)
                && equal_or_none(aunt.samoyeds, 2)
                && equal_or_none(aunt.pomeranians, 3)
                && equal_or_none(aunt.akitas, 0)
                && equal_or_none(aunt.vizslas, 0)
                && equal_or_none(aunt.goldfish, 5)
                && equal_or_none(aunt.trees, 3)
                && equal_or_none(aunt.cars, 2)
                && equal_or_none(aunt.perfumes, 1)
        })
        .unwrap();

    real_aunt.number
}

fn part_2(aunts: &[Aunt]) -> u32 {
    let equal_or_none = |field: Option<u32>, expected: u32| -> bool {
        if let Some(count) = field {
            if count != expected {
                return false;
            }
        }
        true
    };
    let greater_than_or_none = |field: Option<u32>, expected: u32| -> bool {
        if let Some(count) = field {
            if count <= expected {
                return false;
            }
        }
        true
    };
    let fewer_or_none = |field: Option<u32>, expected: u32| -> bool {
        if let Some(count) = field {
            if count >= expected {
                return false;
            }
        }
        true
    };

    let real_aunt = aunts
        .iter()
        .find(|aunt| {
            equal_or_none(aunt.children, 3)
                && greater_than_or_none(aunt.cats, 7)
                && equal_or_none(aunt.samoyeds, 2)
                && fewer_or_none(aunt.pomeranians, 3)
                && equal_or_none(aunt.akitas, 0)
                && equal_or_none(aunt.vizslas, 0)
                && fewer_or_none(aunt.goldfish, 5)
                && greater_than_or_none(aunt.trees, 3)
                && equal_or_none(aunt.cars, 2)
                && equal_or_none(aunt.perfumes, 1)
        })
        .unwrap();

    real_aunt.number
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();

    let aunts: Vec<Aunt> = input.lines().map(|line| parse_line(line)).collect();

    assert_eq!(373, part_1(&aunts));
    assert_eq!(260, part_2(&aunts));
}
