use crate::utils::read_lines;

pub fn part_one(filename: &str) -> i32 {
    let positions = read_lines(filename)
        .nth(0).expect("line missing").expect("line error").split(",")
            .map(|s| s.parse::<i32>().expect("parse error"))
            .collect::<Vec<_>>();

    let position_range = positions.clone().into_iter().min().expect("minimum not found")
        ..=positions.clone().into_iter().max().expect("maximum not found");

    let fuel_consumptions = position_range.clone().into_iter()
        .map(|goal| positions.clone().into_iter()
            .fold(0, |f, p| f + (p - goal).abs()))
        .collect::<Vec<i32>>();

    fuel_consumptions.into_iter().min().expect("minimum not found")
}

pub fn part_two(filename: &str) -> i32 {
    let positions = read_lines(filename)
        .nth(0).expect("line missing").expect("line error").split(",")
            .map(|s| s.parse::<i32>().expect("parse error"))
            .collect::<Vec<_>>();

    let position_range = positions.clone().into_iter().min().expect("minimum not found")
        ..=positions.clone().into_iter().max().expect("maximum not found");

    let fuel_consumptions = position_range.clone().into_iter()
        .map(|goal| positions.clone().into_iter()
            .fold(0, |f, p| f + ((p - goal).abs() + 1)*(p - goal).abs()/2))
        .collect::<Vec<i32>>();

    fuel_consumptions.into_iter().min().expect("minimum not found")
}
