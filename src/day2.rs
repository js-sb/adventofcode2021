use crate::utils::{read_lines,IntoTuple};

pub fn part_one(filename: &str) -> i32 {
    let position = read_lines(filename).into_tuple(" ").iter()
        .fold((0, 0), |mut pos, cmd|
            match &*cmd.0 {
                "forward" => {
                    pos.0 += cmd.1;
                    pos
                },
                "down" => {
                    pos.1 += cmd.1;
                    pos
                },
                "up" => {
                    pos.1 -= cmd.1;
                    pos
                },
                _ => { pos }
            });

    position.0 * position.1
}

pub fn part_two(filename:&str) -> i32 {
    let position = read_lines(filename).into_tuple(" ").iter()
        .fold((0, 0, 0), |mut pos, cmd|
            match &*cmd.0 {
                "forward" => {
                    pos.0 += cmd.1;
                    pos.1 += cmd.1 * pos.2;
                    pos
                },
                "down" => {
                    pos.2 += cmd.1;
                    pos
                },
                "up" => {
                    pos.2 -= cmd.1;
                    pos
                },
                _ => { pos }
            });

    position.0 * position.1
}