use crate::utils::read_lines;

pub fn part_one(filename: &str) -> i32 {
    let lines = read_lines(filename)
        .map(|line| line.expect("unable to read line")
            .split(" ").map(|s| s.to_string()).collect::<Vec<String>>())
        .collect::<Vec<_>>();

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
    gamma * epsilon
}

pub fn part_two(filename:&str) -> i32 {
    let lines = read_lines(filename)
        .map(|line| line.expect("unable to read line")
            .split(" ").map(|s| s.to_string()).collect::<Vec<String>>())
        .collect::<Vec<_>>();

    let mut char_vec_g: Vec<&str> = lines.iter().map(|l| &*l[0]).collect();
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
    let ls1 = i32::from_str_radix(char_vec_g[0], 2).expect("unable to parse");
    let ls2 = i32::from_str_radix(char_vec_e[0], 2).expect("unable to parse");

    ls1 * ls2
}

