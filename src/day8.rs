use itertools::Itertools;
use crate::utils::read_lines;

pub fn part_one(filename: &str) -> i32 {
    let lines = read_lines(filename);
    let line_split = lines
        .map(|l| l.expect("unable to read line")
            .split(" | ")
            .map(|sl| sl.to_string())
            .collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
    line_split.into_iter().fold(0, |num: i32, ls| num + ls[1].split(" ")
        .fold(0, |n: i32, s|
            if s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7 { n + 1 } else { n }))
}

pub fn part_two(filename: &str) -> i64 {
    let lines = read_lines(filename);
    let line_split = lines
        .map(|l| l.expect("unable to read line")
            .split(" | ")
            .map(|sl| sl.to_string())
            .collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

        fn translate_to_number(segment: &str) -> Option<i32> {
            match segment {
                "ABCEFG" => Some(0),
                "CF" => Some(1),
                "ACDEG" => Some(2),
                "ACDFG" => Some(3),
                "BCDF" => Some(4),
                "ABDFG" => Some(5),
                "ABDEFG" => Some(6),
                "ACF" => Some(7),
                "ABCDEFG" => Some(8),
                "ABCDFG" => Some(9),
                _ => None
            }
        }

        let mut result:i64 = 0;
        for ls in line_split.clone() {
            'perm_loop: for permutation in "ABCDEFG".chars()
                .collect::<Vec<char>>().into_iter().permutations(7) {
                let segments = ls[0].split(" ")
                    .map(|d| {
                        let mut segment = d.chars().collect::<Vec<char>>();
                        segment = segment.into_iter().map(|c| match c {
                            'a' => permutation[0],
                            'b' => permutation[1],
                            'c' => permutation[2],
                            'd' => permutation[3],
                            'e' => permutation[4],
                            'f' => permutation[5],
                            'g' => permutation[6],
                            _ => panic!()
                        }).collect();
                        segment.sort_by_key(|x| *x);
                        segment.into_iter().collect::<String>()
                    }).collect::<Vec<String>>();

                for s in segments.iter() {
                    if None == translate_to_number(s) {
                        continue 'perm_loop;
                    }
                }

                let result_segments = ls[1].split(" ")
                    .map(|d| {
                        let mut segment = d.chars().collect::<Vec<char>>();
                        segment = segment.into_iter().map(|c| match c {
                            'a' => permutation[0],
                            'b' => permutation[1],
                            'c' => permutation[2],
                            'd' => permutation[3],
                            'e' => permutation[4],
                            'f' => permutation[5],
                            'g' => permutation[6],
                            _ => panic!()
                        }).collect();
                        segment.sort_by_key(|x| *x);
                        translate_to_number(&segment.into_iter().collect::<String>())
                            .expect("not a valid digit")
                    }).collect::<Vec<i32>>();
                    result += result_segments.into_iter()
                        .fold(0, |num:i64, digit| 10 * num + digit as i64);
            }
        }
        result
}