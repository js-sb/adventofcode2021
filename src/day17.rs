use crate::utils::read_lines;

pub fn part_one(filename: &str) -> i32 {
    let lines = read_lines(filename);
    let string = lines.into_iter().nth(0).unwrap().unwrap()
        .chars().collect::<Vec<char>>()[15..].iter()
        .collect::<String>();
    let mut split = string.split(", y=").into_iter();

    let y_limits = split.nth(1).unwrap().split("..")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();


    let y_vel = y_limits[0].abs().max(y_limits[1].abs()) - 1;


    (1..=y_vel).sum::<i32>()
}

pub fn part_two(filename: &str) -> i32 {
    let lines = read_lines(filename);
    let string = lines.into_iter().nth(0).unwrap().unwrap()
        .chars().collect::<Vec<char>>()[15..].iter()
        .collect::<String>();
    let mut split = string.split(", y=").into_iter();

    let x_limits = split.next().unwrap().split("..")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let y_limits = split.next().unwrap().split("..")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();


    let x_vel_min = ((2.0 * x_limits[0] as f64 + 0.25).sqrt() - 0.5).ceil() as i32;
    let x_vel_max = x_limits[1];
    let y_vel_min = y_limits[0];
    let y_vel_max = y_limits[0].abs().max(y_limits[1].abs()) - 1;


    fn is_plausible(x: i32, y: i32, x_limits: &Vec<i32>, y_limits: &Vec<i32>) -> bool {
        let mut x_pos = 0;
        let mut y_pos = 0;
        let mut x_vel = x;
        let mut y_vel = y;
        loop {
            x_pos += x_vel;
            y_pos += y_vel;

            y_vel -= 1;
            x_vel = (x_vel - 1).max(0);

            if x_limits[0] <= x_pos && x_pos <= x_limits[1] &&
                y_limits[0] <= y_pos && y_pos <= y_limits[1]
            {
                return true;
            } else if x_pos > x_limits[1] || y_pos < y_limits[0]
            {
                return false;
            }
        }
    }

    let mut n_possible_paths = 0;
    for y in y_vel_min..=y_vel_max {
        for x in x_vel_min..=x_vel_max {
            if is_plausible(x, y, &x_limits, &y_limits) {
                n_possible_paths += 1;
            }
        }
    }
    n_possible_paths
}