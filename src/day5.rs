use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Lines};
use crate::utils::read_lines;

fn parse_vents(lines:Lines<BufReader<File>>) -> Vec<((i32, i32),(i32, i32))>{
    let mut vents = vec![];
    for line in lines {
        let vent = line.expect("unable to read line")
            .split(" -> ")
            .map(|t| t.split(",")
                .map(|n| n.parse::<i32>().expect("unable to parse number"))
                .collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
        vents.push(((vent[0][0], vent[0][1]), (vent[1][0], vent[1][1])));
    }
    vents
}

fn tuple_range(t0: (i32, i32), t1: (i32, i32), inlcude_diagonals:bool) -> Vec<(i32, i32)> {
    let mut r = vec![];
    let min_0 = min(t0.0, t1.0);
    let max_0 = max(t0.0, t1.0);
    let min_1 = min(t0.1, t1.1);
    let max_1 = max(t0.1, t1.1);
    if min_0 == max_0 {
        for v_1 in min_1..=max_1 {
            r.push((min_0, v_1));
        }
    } else if min_1 == max_1 {
        for v_0 in min_0..=max_0 {
            r.push((v_0, min_1));
        }
    } else if inlcude_diagonals {
        if t0.0 == min_0 && t0.1 == min_1
            || t0.0 == max_0 && t0.1 == max_1 {
            for i in 0..=max_0 - min_0 {
                r.push((min_0 + i, min_1 + i));
            }
        } else {
            for i in 0..=max_0 - min_0 {
                r.push((min_0 + i, max_1 - i));
            }
        }
    }
    r
}

pub fn part_one(filename: &str) -> i32 {
    let lines = read_lines(filename);
    let vents = parse_vents(lines);

    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    for vent in vents.iter() {
        for v in tuple_range(vent.0, vent.1, false) {
            *map.entry(v).or_insert(0) += 1;
        }
    }

    map.values().fold(0, |s, v| if *v > 1 { s + 1 } else { s })
}

pub fn part_two(filename: &str) -> i32 {
    let lines = read_lines(filename);
    let vents = parse_vents(lines);

    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    for vent in vents.iter() {
        for v in tuple_range(vent.0, vent.1, true) {
            *map.entry(v).or_insert(0) += 1;
        }
    }

    map.values().fold(0, |s, v| if *v > 1 { s + 1 } else { s })
}