use crate::utils::read_lines;

pub fn part_one(filename: &str) -> i32 {
    let lines = read_lines(filename);
    let mat = lines.into_iter()
        .map(|line| line.expect("unable to read line").chars()
            .map(|c| c.to_string().parse::<i32>().expect("unable to parse")).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let mut val = 0;
    for y in 0..(&mat).len() {
        for x in 0..(&mat[0]).len() {
            let u = (vec![-1 + x as i32,1 + x as i32])
                .into_iter()
                .filter(|q| 0 <= *q && *q < (&mat[0]).len() as i32)
                .map(|q| q as usize)
                .into_iter()
                .collect::<Vec<usize>>();
            let v = (vec![-1 + y as i32,1 + y as i32])
                .into_iter()
                .filter(|q| 0 <= *q && *q < (&mat).len() as i32)
                .map(|q| q as usize)
                .into_iter()
                .collect::<Vec<usize>>();

            if v.into_iter().any(|v| mat[v][x] <= mat[y][x])
                || u.into_iter().any(|u| mat[y][u] <= mat[y][x]) {
                continue;
            } else {
                val += 1 + mat[y][x];
            }
        }
    }
    val
}

pub fn part_two(filename: &str) -> i32 {
    let lines = read_lines(filename);
    let mat = lines.into_iter()
        .map(|line| line.expect("unable to read line").chars()
            .map(|c| c.to_string().parse::<i32>().expect("unable to parse")).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let mut lowpoints:Vec<(usize, usize)> = vec![];
    for y in 0..(&mat).len() {
        for x in 0..(&mat[0]).len() {
            let u = (vec![-1 + x as i32,1 + x as i32])
                .into_iter()
                .filter(|q| 0 <= *q && *q < (&mat[0]).len() as i32)
                .map(|q| q as usize)
                .into_iter()
                .collect::<Vec<usize>>();
            let v = (vec![-1 + y as i32,1 + y as i32])
                .into_iter()
                .filter(|q| 0 <= *q && *q < (&mat).len() as i32)
                .map(|q| q as usize)
                .into_iter()
                .collect::<Vec<usize>>();

            if v.into_iter().any(|v| mat[v][x] <= mat[y][x])
                || u.into_iter().any(|u| mat[y][u] <= mat[y][x]) {
                continue;
            } else {
                lowpoints.push((y,x));
            }
        }
    }

    fn fill_basin(pos:(usize,usize), mat: &mut Vec<Vec<i32>>) -> i32 {
        let mut basin_size = 1;
        let cmp_val:i32 = mat[pos.0][pos.1];
        mat[pos.0][pos.1] = 9;

        for t in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let temp_pos = (t.0 + pos.0 as i32, t.1 + pos.1 as i32);
            if 0 > temp_pos.0
                || temp_pos.0 >= mat.len() as i32
                || 0 > temp_pos.1
                || temp_pos.1 >= mat[0].len() as i32
                || mat[temp_pos.0 as usize][temp_pos.1 as usize] == 9
                || mat[temp_pos.0 as usize][temp_pos.1 as usize] < cmp_val
            {
                continue;
            } else {
                basin_size += fill_basin((temp_pos.0 as usize, temp_pos.1 as usize), mat);
            }
        }
        basin_size
    }

    let mut basin_sizes:Vec<i32> = vec![];
    for lp in lowpoints {
        basin_sizes.push(fill_basin(lp, &mut mat.clone()));
    }
    basin_sizes.sort();
    basin_sizes.pop().unwrap() * basin_sizes.pop().unwrap() * basin_sizes.pop().unwrap()
}