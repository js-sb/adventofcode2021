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
        let commands = include_str!("../input/day3.txt").lines()
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
}
