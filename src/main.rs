mod utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;

fn main() {
    println!("==== DAY1 ====");
    println!("Solution Part One\t    {}", day1::part_one("input/day1.txt"));
    println!("Solution Part Two\t    {}", day1::part_two("input/day1.txt"));

    println!("==== DAY2 ====");
    println!("Solution Part One\t    {}", day2::part_one("input/day2.txt"));
    println!("Solution Part Two\t    {}", day2::part_two("input/day2.txt"));

    println!("==== DAY3 ====");
    println!("Solution Part One\t    {}", day3::part_one("input/day3.txt"));
    println!("Solution Part Two\t    {}", day3::part_two("input/day3.txt"));

    println!("==== DAY4 ====");
    println!("Solution Part One\t    {}", day4::part_one("input/day4.txt"));
    println!("Solution Part Two\t    {}", day4::part_two("input/day4.txt"));

    println!("==== DAY5 ====");
    println!("Solution Part One\t    {}", day5::part_one("input/day5.txt"));
    println!("Solution Part Two\t    {}", day5::part_two("input/day5.txt"));

    println!("==== DAY6 ====");
    println!("Solution Part One\t    {}", day6::part_one("input/day6.txt"));
    println!("Solution Part Two\t    {}", day6::part_two("input/day6.txt"));

    println!("==== DAY7 ====");
    println!("Solution Part One\t    {}", day7::part_one("input/day7.txt"));
    println!("Solution Part Two\t    {}", day7::part_two("input/day7.txt"));

    println!("==== DAY8 ====");
    println!("Solution Part One\t    {}", day8::part_one("input/day8.txt"));
    // println!("Solution Part Two\t    {}",day8::part_two("input/day8.txt")); // TODO optimize, super slow

    println!("==== DAY9 ====");
    println!("Solution Part One\t    {}", day9::part_one("input/day9.txt"));
    println!("Solution Part Two\t    {}", day9::part_two("input/day9.txt"));

    println!("==== DAY10 ====");
    println!("Solution Part One\t    {}", day10::part_one("input/day10.txt"));
    println!("Solution Part Two\t    {}", day10::part_two("input/day10.txt"));

    println!("==== DAY11 ====");
    println!("Solution Part One\t    {}", day11::part_one("input/day11.txt"));
    println!("Solution Part Two\t    {}", day11::part_two("input/day11.txt"));

    println!("==== DAY12 ====");
    println!("Solution Part One\t    {}", day12::part_one("input/day12.txt"));
    println!("Solution Part Two\t    {}", day12::part_two("input/day12.txt"));
}
