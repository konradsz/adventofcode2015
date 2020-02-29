use multimap::MultiMap;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

const STARTING_MOLECULE: &str = "CRnCaCaCaSiRnBPTiMgArSiRnSiRnMgArSiRnCaFArTiTiBSiThFYCaFArCaCaSiT\
hCaPBSiThSiThCaCaPTiRnPBSiThRnFArArCaCaSiThCaSiThSiRnMgArCaPTiBPRnFArSiThCaSiRnFArBCaSiRnCaPRnFArP\
MgYCaFArCaPTiTiTiBPBSiThCaPTiBPBSiRnFArBPBSiRnCaFArBPRnSiRnFArRnSiRnBFArCaFArCaCaCaSiThSiThCaCaPBP\
TiTiRnFArCaPTiBSiAlArPBCaCaCaCaCaSiRnMgArCaSiThFArThCaSiThCaSiRnCaFYCaSiRnFYFArFArCaSiRnFYFArCaSiR\
nBPMgArSiThPRnFArCaSiRnFArTiRnSiRnFYFArCaSiRnBFArCaSiRnTiMgArSiThCaSiThCaFArPRnFArSiRnFArTiTiTiTiB\
CaCaSiRnCaCaFYFArSiThCaPTiBPTiBCaSiThSiRnMgArCaF";

fn part_1(replacements: &MultiMap<String, String>) -> usize {
    let mut molecules = HashSet::new();
    for (input, outputs) in replacements {
        for (index, _) in STARTING_MOLECULE.match_indices(input) {
            for output in outputs {
                let molecule = [
                    &STARTING_MOLECULE[0..index],
                    &output,
                    &STARTING_MOLECULE[index + input.len()..],
                ]
                .concat();
                molecules.insert(molecule);
            }
        }
    }

    molecules.len()
}

fn part_2(replacements: &MultiMap<String, String>) -> usize {
    let mut molecule = String::from(STARTING_MOLECULE);

    let replacements = reverse_map(replacements);
    let mut steps = 0;
    while molecule != "e" {
        for (from, to) in replacements.iter() {
            if let Some(index) = molecule.find(from) {
                molecule = [&molecule[0..index], &to, &molecule[index + from.len()..]]
                    .concat()
                    .to_string();
                steps += 1;
            }
        }
    }

    steps
}

fn parse_input(input: &str) -> MultiMap<String, String> {
    let replacement_regex = Regex::new(r"^(?P<input>\S+) => (?P<output>\S+)$").unwrap();

    let mut replacements = MultiMap::new();
    for line in input.lines() {
        let caps = replacement_regex.captures(line).unwrap();
        replacements.insert(String::from(&caps["input"]), String::from(&caps["output"]));
    }

    replacements
}

fn reverse_map(map: &MultiMap<String, String>) -> HashMap<String, String> {
    let mut reversed_map = HashMap::new();

    for (input, outputs) in map {
        for output in outputs {
            reversed_map.insert(output.clone(), input.clone());
        }
    }

    reversed_map
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();

    let replacements = parse_input(input);

    assert_eq!(535, part_1(&replacements));
    assert_eq!(212, part_2(&replacements));
}
