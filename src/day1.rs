use crate::utils::read_lines;

pub fn part_one(filename:&str) -> i32{
    let numbers = read_lines(filename)
        .map(|l| l.expect("unable to read line")
            .parse::<i32>().expect("unable to parse integer"))
        .collect::<Vec<i32>>();

    numbers.iter().zip(numbers[1..].iter()).filter(|n| n.1 > n.0).count() as i32
}

pub fn part_two(filename:&str) -> i32 {
    let numbers = read_lines(filename)
        .map(|l| l.expect("unable to read line")
            .parse::<i32>().expect("unable to parse integer"))
        .collect::<Vec<i32>>();

    numbers.iter().zip(numbers[3..].iter()).filter(|n| n.1 > n.0).count() as i32
}