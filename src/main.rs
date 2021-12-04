extern crate regex;
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
        let mut position:(i32,i32) = (0, 0);
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
        let mut aim:i32 = 0;
        position = (0, 0);
        for command in &commands {
            if let Ok(delta) = command[1].parse::<i32>() {
                match command[0] {
                    "forward" => {
                        position.0 += delta;
                        position.1 += delta * aim
                    },
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
        let mut b = vec!(0,0,0,0,0,0,0,0,0,0,0,0);
        for line in &lines {
            for (i,c) in line[0].chars().enumerate(){
                if c=='1' {
                    b[i] += 1;
                }
            }
        }
        let mut gamma = 0;
        let mut epsilon = 0;

        for i in 0..12 {
            if (2 * b[11-i]) >= lines.len(){
                gamma += 1<<i;
            }else{
                epsilon += 1<<i;
            }
        }
        println!("{}", gamma * epsilon);

        println!("--- Solution Part Two ---");

        let mut char_vec_g:Vec<_> =lines.iter().map(|l| l[0]).collect();
        let mut char_vec_e=char_vec_g.clone();
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
        let lines = include_str!("../input/day3.txt").lines()
            .map(|line| line.split(" ").collect::<Vec<&str>>()).collect::<Vec<_>>();

        println!("--- Solution Part One ---");
        let mut b = vec!(0,0,0,0,0,0,0,0,0,0,0,0);
        for line in &lines {
            for (i,c) in line[0].chars().enumerate(){
                if c=='1' {
                    b[i] += 1;
                }
            }
        }
        let mut gamma = 0;
        let mut epsilon = 0;

        for i in 0..12 {
            if (2 * b[11-i]) >= lines.len(){
                gamma += 1<<i;
            }else{
                epsilon += 1<<i;
            }
        }
        println!("{}", gamma * epsilon);

        println!("--- Solution Part Two ---");

        let mut char_vec_g:Vec<_> =lines.iter().map(|l| l[0]).collect();
        let mut char_vec_e=char_vec_g.clone();
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
}
