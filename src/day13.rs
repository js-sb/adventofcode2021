use std::{cmp::max, collections::HashSet};
use crate::utils::read_lines;

fn fold_set(set: &mut HashSet<(i32, i32)>, folding: (i32, i32)) {
    set.clone().into_iter().for_each(|key| match key {
        (x, y) if folding.0 != 0 && x > folding.0 => {
            set.remove(&key);
            set.insert((2 * folding.0 - x, y));
        }
        (x, y) if folding.1 != 0 && y > folding.1 => {
            set.remove(&key);
            set.insert((x, 2 * folding.1 - y));
        }
        _ => (),
    });
}

pub fn part_one(filename: &str) -> i32 {
    let lines = read_lines(filename);
    let mut dots = HashSet::new();
    let mut folds = vec![];
    lines.for_each(|l| match l.unwrap().as_str() {
        s if s.chars().next() == Some('f') => {
            s.split("fold along ").for_each(|f| {
                if !f.is_empty() {
                    let t = f.split("=").map(|s| s.to_string()).collect::<Vec<String>>();
                    match t[0].as_str() {
                        "x" => folds.push((t[1].parse::<i32>().unwrap(), 0)),
                        "y" => folds.push((0, t[1].parse::<i32>().unwrap())),
                        _ => ()
                    }
                }
            });
        }
        s if !s.trim().is_empty() => {
            let t = s.split(",").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            dots.insert((t[0], t[1]));
        }
        _ => (),
    });

    fold_set(&mut dots, folds[0]);

    dots.len() as i32
}

pub fn part_two(filename: &str) -> String {
    let lines = read_lines(filename);
    let mut dots = HashSet::new();
    let mut folds = vec![];
    lines.for_each(|l| match l.unwrap().as_str() {
        s if s.chars().next() == Some('f') => {
            s.split("fold along ").for_each(|f| {
                if !f.is_empty() {
                    let t = f.split("=").map(|s| s.to_string()).collect::<Vec<String>>();
                    match t[0].as_str() {
                        "x" => folds.push((t[1].parse::<i32>().unwrap(), 0)),
                        "y" => folds.push((0, t[1].parse::<i32>().unwrap())),
                        _ => ()
                    }
                }
            });
        }
        s if !s.trim().is_empty() => {
            let t = s.split(",").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            dots.insert((t[0], t[1]));
        }
        _ => (),
    });


    folds.into_iter().for_each(|f| fold_set(&mut dots, f));

    fn print_letters(dots: HashSet<(i32, i32)>) -> String {
        let limits = dots.clone().into_iter()
            .fold((0, 0), |l, d| (max(l.0, d.0), max(l.1, d.1)));

        let mut result = vec![vec![false; limits.0 as usize + 1]; limits.1 as usize + 1];

        dots.into_iter().for_each(|(x, y)| result[y as usize][x as usize] = true);

        let mut result_str = "\n".to_string();
        (0..3).into_iter().for_each(|i| {
            result[2 * i].clone().into_iter()
                .zip(result[2 * i + 1].clone().into_iter())
                .for_each(|l| match l {
                    (true, true) => result_str
                        .push_str(String::from_utf16(&[0x2588])
                            .unwrap().as_str()),
                    (true, false) => result_str
                        .push_str(String::from_utf16(&[0x2580])
                            .unwrap().as_str()),
                    (false, true) => result_str
                        .push_str(String::from_utf16(&[0x2584])
                            .unwrap().as_str()),
                    _ => result_str.push_str(" "),
                });
            result_str.push_str("\n");
        });

        result_str
    }

    print_letters(dots)
}