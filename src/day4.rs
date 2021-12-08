use std::fs::File;
use std::io::{BufReader, Lines};
use crate::utils::read_lines;

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

fn parse_boards(lines: Lines<BufReader<File>>) -> (Vec<i32>, Vec<Vec<i32>>) {
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
    (draw, boards)
}

pub fn part_one(filename: &str) -> i32 {
    let lines = read_lines(filename);
    let (draw, boards) = parse_boards(lines);


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

    winner_config.0.into_iter().sum::<i32>() * winner_config.1
}

pub fn part_two(filename: &str) -> i32 {
    let lines = read_lines(filename);
    let (draw, mut boards) = parse_boards(lines);


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
    loser_config.0.into_iter().sum::<i32>() * loser_config.1
}
