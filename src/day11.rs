use crate::utils::read_lines;
// use image::{ImageBuffer, RgbImage};
// use image::imageops::{resize, FilterType::Gaussian};


fn get_index(x: i32, y: i32) -> usize {
    x as usize + y as usize * 10
}

/*fn save_snapshot(array: &Vec<i32>, index: i32) {
    let mut image: RgbImage = ImageBuffer::new(10, 10);
    (0..10).into_iter().for_each(|y|
        (0..10).into_iter().for_each(|x| {
            let power = array[get_index(x as i32, y as i32)];
            *image.get_pixel_mut(x, y) =
                match power {
                    p if p > 10 => image::Rgb([255, 255, 255]),
                    9 => image::Rgb([0, 255, 200]),
                    10 => image::Rgb([0, 0, 255]),
                    _ => image::Rgb([0, power as u8 * 20, power as u8 * 28])
                };
        }));
    image = resize(&image, 200, 200, Gaussian);
    image.save(format!("out{:>6}.png", index)).unwrap();
}*/


fn blink(x: i32, y: i32, dumbos: &mut Vec<i32>, flashes: &mut i32) {
    dumbos[get_index(x, y)] += 1;
    *flashes += 1;
    for dy in [-1, 0, 1] {
        for dx in [-1, 0, 1] {
            if 0 <= dx + x && dx + x < 10
                && 0 <= dy + y && dy + y < 10
                && !(dx == 0 && dy == 0) {
                if dumbos[get_index(x + dx, y + dy)] != 10 {
                    dumbos[get_index(x + dx, y + dy)] += 1;
                }
                if dumbos[get_index(x + dx, y + dy)] == 10 {
                    blink(x + dx, y + dy, dumbos, flashes);
                }
            }
        }
    }
}

pub fn part_one(filename: &str) -> i32 {
    let lines = read_lines(filename);
    let mut dumbo_array = lines.into_iter()
        .fold(vec![], |mut dumbos: Vec<i32>, line|
            {
                dumbos.append(
                    line.unwrap().chars()
                        .map(|c|
                            c.to_string().parse::<i32>().unwrap())
                        .collect::<Vec<i32>>().as_mut()
                );
                dumbos
            },
        );

    let mut flashes = 0;

    for _ in 0..100 {
        (0..10).into_iter().for_each(|y|
            (0..10).into_iter().for_each(|x|
                {
                    dumbo_array[get_index(x, y)] += 1;
                }));

        for y in 0..10 {
            for x in 0..10 {
                if dumbo_array[get_index(x, y)] == 10 {
                    blink(x, y, &mut dumbo_array, &mut flashes);
                }
            }
        }

        (0..10).into_iter().for_each(|y|
            (0..10).into_iter().for_each(|x|
                {
                    if dumbo_array[get_index(x, y)] >= 10 {
                        dumbo_array[get_index(x, y)] = 0;
                    }
                }
            )
        );
    }
    flashes
}

pub fn part_two(filename: &str) -> i32 {
    let lines = read_lines(filename);
    let mut dumbo_array = lines.into_iter()
        .fold(vec![], |mut dumbos: Vec<i32>, line|
            {
                dumbos.append(
                    line.unwrap().chars()
                        .map(|c|
                            c.to_string().parse::<i32>().unwrap())
                        .collect::<Vec<i32>>().as_mut()
                );
                dumbos
            },
        );

    let mut flashes = 0;

    for i in 0..300 {
        (0..10).into_iter().for_each(|y|
            (0..10).into_iter().for_each(|x|
                {
                    dumbo_array[get_index(x, y)] += 1;
                }));

        for y in 0..10 {
            for x in 0..10 {
                if dumbo_array[get_index(x, y)] == 10 {
                    blink(x, y, &mut dumbo_array, &mut flashes);
                }
            }
        }

        (0..10).into_iter().for_each(|y|
            (0..10).into_iter().for_each(|x|
                {
                    if dumbo_array[get_index(x, y)] >= 10 {
                        dumbo_array[get_index(x, y)] = 0;
                    }
                }
            )
        );

        if dumbo_array.iter().sum::<i32>() == 0 {
            flashes = i + 1;
            break;
        }

    }
    flashes
}