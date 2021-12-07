extern crate regex;

use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};
// use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    println!("==== Day 1 ====");
    {
        let file = File::open("input/day1.txt").unwrap();

        let reader = BufReader::new(file);
        let numbers = reader.lines()
            .map(|l| l.unwrap().parse::<i32>().unwrap()).collect::<Vec<i32>>();

        println!("--- Solution Part One ---");
        let mut n_increasing: i32 = 0;
        for i in 1..numbers.len() {
            if numbers[i] > numbers[i - 1] {
                n_increasing += 1;
            }
        }
        println!("{}", n_increasing);

        println!("--- Solution Part Two ---");
        let mut n_increasing: i32 = 0;
        for i in 3..numbers.len() {
            if numbers[i] > numbers[i - 3] {
                n_increasing += 1;
            }
        }
        println!("{}", n_increasing);
    }

    println!("==== Day 2 ====");
    {
        let commands = include_str!("../input/day2.txt").lines()
            .map(|line| line.split(" ").collect::<Vec<&str>>()).collect::<Vec<_>>();

        println!("--- Solution Part One ---");
        let mut position: (i32, i32) = (0, 0);
        for command in &commands {
            if let Ok(delta) = command[1].parse::<i32>() {
                match command[0] {
                    "forward" => position.0 += delta,
                    "down" => position.1 += delta,
                    "up" => position.1 -= delta,
                    _ => panic!(),
                }
            }
        }
        println!("{}", position.0 * position.1);

        println!("--- Solution Part Two ---");
        let mut aim: i32 = 0;
        position = (0, 0);
        for command in &commands {
            if let Ok(delta) = command[1].parse::<i32>() {
                match command[0] {
                    "forward" => {
                        position.0 += delta;
                        position.1 += delta * aim
                    }
                    "down" => aim += delta,
                    "up" => aim -= delta,
                    _ => panic!(),
                }
            }
        }
        println!("{}", position.0 * position.1);
    }

    println!("==== Day 3 ====");
    {
        let lines = include_str!("../input/day3.txt").lines()
            .map(|line| line.split(" ").collect::<Vec<&str>>()).collect::<Vec<_>>();

        println!("--- Solution Part One ---");
        let mut b = vec!(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
        for line in &lines {
            for (i, c) in line[0].chars().enumerate() {
                if c == '1' {
                    b[i] += 1;
                }
            }
        }
        let mut gamma = 0;
        let mut epsilon = 0;

        for i in 0..12 {
            if (2 * b[11 - i]) >= lines.len() {
                gamma += 1 << i;
            } else {
                epsilon += 1 << i;
            }
        }
        println!("{}", gamma * epsilon);

        println!("--- Solution Part Two ---");

        let mut char_vec_g: Vec<_> = lines.iter().map(|l| l[0]).collect();
        let mut char_vec_e = char_vec_g.clone();
        for i in 0..char_vec_g[0].len() {
            let filter_g = char_vec_g.iter()
                .fold(0, |n, s| if s.chars().nth(i) == Some('1') { n + 1 } else { n }) * 2
                >= char_vec_g.len();
            char_vec_g = char_vec_g.iter().cloned()
                .filter(|s| (s.chars().nth(i) == Some('1')) == filter_g).collect::<Vec<_>>();

            if char_vec_g.len() == 1 {
                break;
            }
        }
        for i in 0..char_vec_e[0].len() {
            let filter_e = char_vec_e.iter()
                .fold(0, |n, s| if s.chars().nth(i) == Some('1') { n + 1 } else { n }) * 2
                < char_vec_e.len();
            char_vec_e = char_vec_e.iter().cloned()
                .filter(|s| (s.chars().nth(i) == Some('1')) == filter_e).collect::<Vec<_>>();

            if char_vec_e.len() == 1 {
                break;
            }
        }
        if let Ok(ls1) = i32::from_str_radix(char_vec_g[0], 2) {
            if let Ok(ls2) = i32::from_str_radix(char_vec_e[0], 2) {
                println!("{:?}", ls1 * ls2);
            }
        }
    }

    println!("==== Day 4 ====");
    {
        let lines =
            BufReader::new(File::open("input/day4.txt")
                .expect("unable to open file"))
                .lines();


        let mut draw: Vec<i32> = vec![];
        let mut boards: Vec<Vec<i32>> = vec![];
        let mut current_board: Vec<i32> = vec![];
        for (i, line) in lines.enumerate() {
            let line = line.expect("unable to read line");

            if i == 0 {
                draw = line
                    .trim()
                    .split(",")
                    .map(|s| s.parse::<i32>().expect("unable to parse number"))
                    .collect::<Vec<i32>>();
            } else if i == 1 {} else if line.trim().is_empty() {
                boards.push(current_board.clone());
                current_board.clear();
            } else {
                current_board.append(line
                    .trim()
                    .split_whitespace()
                    .map(|n| n.parse::<i32>().expect("unable to parse number"))
                    .collect::<Vec<i32>>().as_mut())
            }
        }
        boards.push(current_board.clone());
        current_board.clear();

        fn check_bingo(draw: &[i32], board: &[i32]) -> (bool, Vec<i32>) {
            let hits = board.into_iter()
                .map(|b| draw.iter().any(|d| d == b)).collect::<Vec<bool>>();

            if hits[0..5].iter().fold(0, |s, h| s + *h as i32) == 5 ||
                hits[5..10].iter().fold(0, |s, h| s + *h as i32) == 5 ||
                hits[10..15].iter().fold(0, |s, h| s + *h as i32) == 5 ||
                hits[15..20].iter().fold(0, |s, h| s + *h as i32) == 5 ||
                hits[20..25].iter().fold(0, |s, h| s + *h as i32) == 5 ||

                [0, 5, 10, 15, 20].into_iter().fold(0, |s, i| s + hits[i] as i32) == 5 ||
                [1, 6, 11, 16, 21].into_iter().fold(0, |s, i| s + hits[i] as i32) == 5 ||
                [2, 7, 12, 17, 22].into_iter().fold(0, |s, i| s + hits[i] as i32) == 5 ||
                [3, 8, 13, 18, 23].into_iter().fold(0, |s, i| s + hits[i] as i32) == 5 ||
                [4, 9, 14, 19, 24].into_iter().fold(0, |s, i| s + hits[i] as i32) == 5

            // [0,6,12,18,24].into_iter().fold(0, |s, i| s + hits[i] as i32) == 5 ||
            // [4,8,12,16,20].into_iter().fold(0, |s, i| s + hits[i] as i32) == 5
            {
                (true,
                 board.clone().into_iter()
                     .map(|b| if draw.iter().any(|d| d == b) { 0 } else { *b })
                     .collect::<Vec<i32>>()
                )
            } else {
                (false, vec![])
            }
        }

        println!("--- Solution Part One ---");
        let mut winner_config: (Vec<i32>, i32) = (vec![], 0);
        'draw_loop: for i in 0..draw.len() {
            for board in boards.iter() {
                let (bingo, w_b) = check_bingo(&draw[0..=i], board);
                if bingo {
                    winner_config = (w_b, draw[i]);
                    break 'draw_loop;
                }
            }
        }
        println!("{:?}", winner_config.0.into_iter().sum::<i32>() * winner_config.1);


        println!("--- Solution Part Two ---");
        let mut loser_config: (Vec<i32>, i32) = (vec![], 0);
        for i in 0..draw.len() {
            if boards.len() == 1 {
                let (bingo, l_b) = check_bingo(&draw[0..=i], &boards[0]);
                if bingo {
                    loser_config = (l_b, draw[i]);
                    break;
                }
            } else {
                boards = boards.into_iter()
                    .filter(|b| (false, vec![]) == check_bingo(&draw[0..=i], b))
                    .collect::<Vec<Vec<i32>>>();
            }
        }
        println!("{:?}", loser_config.0.into_iter().sum::<i32>() * loser_config.1);
    }

    println!("==== Day 5 ====");
    {
        let lines =
            BufReader::new(File::open("input/day5.txt")
                .expect("unable to open file"))
                .lines();


        let mut vents = vec![];
        for line in lines {
            let vent = line.expect("unable to read line")
                .split(" -> ")
                .map(|t| t.split(",")
                    .map(|n| n.parse::<i32>().expect("unable to parse number"))
                    .collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
            vents.push(((vent[0][0], vent[0][1]), (vent[1][0], vent[1][1])));
        }


        println!("--- Solution Part One ---");
        {
            fn tuple_range(t0: (i32, i32), t1: (i32, i32)) -> Vec<(i32, i32)> {
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
                }
                r
            }

            let mut map: HashMap<(i32, i32), i32> = HashMap::new();

            for vent in vents.iter() {
                for v in tuple_range(vent.0, vent.1) {
                    // println!("\t{:?}", v);
                    *map.entry(v).or_insert(0) += 1;
                }
            }

            let danger_zones = map.values().fold(0, |s, v| if *v > 1 { s + 1 } else { s });
            println!("{:?}", danger_zones);
        }

        println!("--- Solution Part Two ---");
        {
            fn tuple_range(t0: (i32, i32), t1: (i32, i32)) -> Vec<(i32, i32)> {
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
                } else if t0.0 == min_0 && t0.1 == min_1
                    || t0.0 == max_0 && t0.1 == max_1 {
                    for i in 0..=max_0 - min_0 {
                        r.push((min_0 + i, min_1 + i));
                    }
                } else {
                    for i in 0..=max_0 - min_0 {
                        r.push((min_0 + i, max_1 - i));
                    }
                }
                r
            }

            let mut map: HashMap<(i32, i32), i32> = HashMap::new();

            for vent in vents.iter() {
                for v in tuple_range(vent.0, vent.1) {
                    // println!("\t{:?}", v);
                    *map.entry(v).or_insert(0) += 1;
                }
            }

            let danger_zones = map.values().fold(0, |s, v| if *v > 1 { s + 1 } else { s });
            println!("{:?}", danger_zones);
        }
    }


    println!("==== Day 6 ====");
    {
        let fishies =
            BufReader::new(File::open("input/day6.txt")
                // BufReader::new(File::open("input/example.txt")
                .expect("unable to open file"))
                .lines().nth(0).expect("line missing").expect("line error").split(",")
                .map(|s| s.parse::<i64>().expect("parse error"))
                .collect::<VecDeque<_>>();

        let mut lantern_fish:VecDeque<i64> = VecDeque::from(vec![
            fishies.clone().into_iter().filter(|d| *d == 0).count() as i64,
            fishies.clone().into_iter().filter(|d| *d == 1).count() as i64,
            fishies.clone().into_iter().filter(|d| *d == 2).count() as i64,
            fishies.clone().into_iter().filter(|d| *d == 3).count() as i64,
            fishies.clone().into_iter().filter(|d| *d == 4).count() as i64,
            fishies.clone().into_iter().filter(|d| *d == 5).count() as i64,
            fishies.clone().into_iter().filter(|d| *d == 6).count() as i64,
            fishies.clone().into_iter().filter(|d| *d == 7).count() as i64,
            fishies.clone().into_iter().filter(|d| *d == 8).count() as i64]);

        println!("--- Solution Part One ---");
        for _ in 0..80 {
            let reproductive_lantern_fish = lantern_fish.pop_front().expect("no entries");
            lantern_fish[6] += reproductive_lantern_fish;
            lantern_fish.push_back(reproductive_lantern_fish);

        }
        println!("{}", lantern_fish.clone().into_iter().sum::<i64>());

        println!("--- Solution Part Two ---");
        for _ in 80..256 {
            let reproductive_lantern_fish = lantern_fish.pop_front().expect("no entries");
            lantern_fish[6] += reproductive_lantern_fish;
            lantern_fish.push_back(reproductive_lantern_fish);

        }
        println!("{}", lantern_fish.clone().into_iter().sum::<i64>());
    }

    println!("==== Day 7 ====");
    {
        let positions =
            BufReader::new(File::open("input/day7.txt")
                // BufReader::new(File::open("input/example.txt")
                .expect("unable to open file"))
                .lines().nth(0).expect("line missing").expect("line error").split(",")
                .map(|s| s.parse::<i32>().expect("parse error"))
                .collect::<Vec<_>>();

        let position_range = positions.clone().into_iter().min().expect("minimum not found")
            ..=positions.clone().into_iter().max().expect("maximum not found");

        println!("--- Solution Part One ---");
        let fuel_consumptions = position_range.clone().into_iter()
            .map(|goal| positions.clone().into_iter()
                .fold(0, |f, p| f + (p - goal).abs()))
            .collect::<Vec<i32>>();

        println!("{:?}",fuel_consumptions.into_iter().min().expect("minimum not found"));
        println!("--- Solution Part Two ---");
        let fuel_consumptions = position_range.clone().into_iter()
            .map(|goal| positions.clone().into_iter()
                .fold(0, |f, p| f + ((p - goal).abs() + 1)*(p - goal).abs()/2))
            .collect::<Vec<i32>>();

        println!("{:?}",fuel_consumptions.into_iter().min().expect("minimum not found"));

    }
}
