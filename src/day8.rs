use std::collections::{HashMap, HashSet};
use crate::utils::read_lines;

fn could_be_zero() -> bool{

    false
}

pub fn part_one(filename: &str) -> i32 {
    let lines = read_lines(filename);
    let line_split = lines
        .map(|l| l.expect("unable to read line")
        .split(" | ")
            .map(|sl | sl.to_string())
            .collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    let mut dictionary:HashMap<char, HashSet<char>> = HashMap::new();
    dictionary.insert('a', "ABCDEFG".chars().collect());
    dictionary.insert('b', "ABCDEFG".chars().collect());
    dictionary.insert('c', "ABCDEFG".chars().collect());
    dictionary.insert('d', "ABCDEFG".chars().collect());
    dictionary.insert('e', "ABCDEFG".chars().collect());
    dictionary.insert('f', "ABCDEFG".chars().collect());
    dictionary.insert('g', "ABCDEFG".chars().collect());


    let mut seven_segment: HashMap<i32, HashSet<char>> = HashMap::new();
    seven_segment.insert(0, "ABCEFG".chars().collect());
    seven_segment.insert(1, "CF".chars().collect());
    seven_segment.insert(2, "ACDEG".chars().collect());
    seven_segment.insert(3, "ACDFG".chars().collect());
    seven_segment.insert(4, "BCDF".chars().collect());
    seven_segment.insert(5, "ABDFG".chars().collect());
    seven_segment.insert(6, "ABDEFG".chars().collect());
    seven_segment.insert(7, "ACF".chars().collect());
    seven_segment.insert(8, "ABCDEFG".chars().collect());
    seven_segment.insert(9, "ABCDFG".chars().collect());


    for ls in line_split.clone() {
        let inputs = ls[0].split(" ")
            .map(|d| d.to_string())
            .collect::<Vec<String>>();

        let mut dict = dictionary.clone();
        for _ in inputs.clone().into_iter() {
            for input in inputs.clone().into_iter() {
                // let input = inputs.pop().expect("");
                let input_set: HashSet<char> = input.chars()
                    .fold(HashSet::new(), |mut s, c|
                        {
                            s.extend(dict.get(&c).expect("no entry"));
                            s
                        });

                let mut number_set: HashSet<char> = HashSet::new();
                for number in seven_segment.values() {
                    if number.len() == input.len()
                        && input_set.is_superset(number) {
                        // println!("input {:?}: {:?},number {:?}: {:?}", input.len(), input_set, number.len(), number);
                        number_set.extend(number);
                    }
                }

                // println!("{:?}", number_set);
                for c in input.chars() {
                    let solution: HashSet<char> = dict.get(&c).expect("no entry")
                        .iter().cloned().collect::<HashSet<char>>()
                        .intersection(&number_set).cloned().collect::<HashSet<char>>();
                    *dict.get_mut(&c).expect("no entry") = solution;
                }
                println!("{:?}", dict);
                println!();
            }
        }


        break;
    }
    0
}

pub fn part_two(filename:&str) -> i32 {
    let lines = read_lines(filename);
    0
}