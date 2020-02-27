#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::fs;

struct Reindeer {
    velocity: u32,
    fly_time: u32,
    rest_time: u32,

    fly_time_left: u32,
    rest_time_left: u32,
    traveled_distance: u32,
    score: u32,
}

impl Reindeer {
    fn new(velocity: u32, fly_time: u32, rest_time: u32) -> Self {
        Reindeer {
            velocity,
            fly_time,
            rest_time,
            fly_time_left: fly_time,
            rest_time_left: 0,
            traveled_distance: 0,
            score: 0,
        }
    }

    fn emulate_one_second(&mut self) {
        if self.fly_time_left > 0 {
            self.fly_time_left -= 1;
            self.traveled_distance += self.velocity;

            if self.fly_time_left == 0 {
                self.rest_time_left += self.rest_time;
            }
        } else if self.rest_time_left > 0 {
            self.rest_time_left -= 1;

            if self.rest_time_left == 0 {
                self.fly_time_left += self.fly_time;
            }
        }
    }
}

fn parse_line(input: &str) -> Reindeer {
    lazy_static! {
        static ref ENTRY_RE: Regex =
            Regex::new(r"^.* can fly (?P<velocity>\d+) km/s for (?P<fly_time>\d+) seconds, but then must rest for (?P<rest_time>\d+) seconds.$")
            .unwrap();
    }
    let caps = ENTRY_RE.captures(input).unwrap();

    let velocity = caps["velocity"].parse::<u32>().unwrap();
    let fly_time = caps["fly_time"].parse::<u32>().unwrap();
    let rest_time = caps["rest_time"].parse::<u32>().unwrap();

    Reindeer::new(velocity, fly_time, rest_time)
}

fn emulate_time_for_reindeers(seconds: u32, reindeers: &mut Vec<Reindeer>) {
    for _ in 0..seconds {
        reindeers
            .iter_mut()
            .for_each(|reindeer| reindeer.emulate_one_second());

        award_score_for_furthest(reindeers);
    }
}

fn award_score_for_furthest(reindeers: &mut Vec<Reindeer>) {
    let current_max = reindeers
        .iter_mut()
        .map(|reindeer| reindeer.traveled_distance)
        .max()
        .unwrap();
    reindeers.iter_mut().for_each(|reindeer| {
        if reindeer.traveled_distance == current_max {
            reindeer.score += 1;
        }
    });
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();

    let mut reindeers: Vec<Reindeer> = input.lines().map(|line| parse_line(line)).collect();
    emulate_time_for_reindeers(2_503, &mut reindeers);

    assert_eq!(
        2_696,
        reindeers
            .iter()
            .map(|reindeer| reindeer.traveled_distance)
            .max()
            .unwrap()
    );
    assert_eq!(
        1_084,
        reindeers
            .iter()
            .map(|reindeer| reindeer.score)
            .max()
            .unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reindeers() {
        let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\n \
                     Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

        let mut reindeers: Vec<Reindeer> = input.lines().map(|line| parse_line(line)).collect();
        emulate_time_for_reindeers(1_000, &mut reindeers);

        assert_eq!(1_120, reindeers[0].traveled_distance);
        assert_eq!(1_056, reindeers[1].traveled_distance);

        assert_eq!(312, reindeers[0].score);
        assert_eq!(689, reindeers[1].score);
    }
}
