use std::collections::VecDeque;
use crate::utils::read_lines;

pub fn part_one(filename: &str) -> i64 {
    let fishies = read_lines(filename)
        .nth(0).expect("line missing").expect("line error").split(",")
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

    for _ in 0..80 {
        let reproductive_lantern_fish = lantern_fish.pop_front().expect("no entries");
        lantern_fish[6] += reproductive_lantern_fish;
        lantern_fish.push_back(reproductive_lantern_fish);
    }

    lantern_fish.clone().into_iter().sum::<i64>()
}

pub fn part_two(filename: &str) -> i64 {
    let fishies = read_lines(filename)
        .nth(0).expect("line missing").expect("line error").split(",")
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

    for _ in 0..256 {
        let reproductive_lantern_fish = lantern_fish.pop_front().expect("no entries");
        lantern_fish[6] += reproductive_lantern_fish;
        lantern_fish.push_back(reproductive_lantern_fish);
    }

    lantern_fish.clone().into_iter().sum::<i64>()
}