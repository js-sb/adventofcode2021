use std::collections::HashMap;
use crate::utils::read_lines;

pub fn part_one(filename: &str) -> i32 {
    let lines = read_lines(filename);

    let mut pairs = HashMap::new();
    let mut rules = HashMap::new();
    let mut letters = HashMap::new();
    for (i, line) in lines.enumerate() {
        if i == 0 {
            let l0 = line.unwrap().chars().into_iter().collect::<Vec<char>>();
            l0[0..].iter().zip(l0[1..].iter())
                .for_each(|p| *pairs.entry(format!("{}{}", p.0, p.1)).or_insert(0) += 1);
            *letters.entry(l0[0]).or_insert(0) += 1;
            *letters.entry(*l0.last().unwrap()).or_insert(0) += 1;
        } else if i != 1 {
            let rule = line.unwrap().split(" -> ")
                .into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
            let key = rule[0].chars().into_iter().collect::<Vec<char>>();
            let insert_char = rule[1].chars().next().unwrap();

            rules.insert(key.iter().collect::<String>(),
                         ([key[0], insert_char].iter().collect::<String>(),
                          [insert_char, key[1]].iter().collect::<String>()));
        }
    }

    let mut pairs2 = HashMap::new();

    let mut p1 = &mut pairs;
    let mut p2 = &mut pairs2;
    for _ in 0..10 {
        p1.iter().for_each(|(key, number)| {
            let new_keys = rules.get(key).unwrap();
            *p2.entry(new_keys.0.clone()).or_insert(0) += *number;
            *p2.entry(new_keys.1.clone()).or_insert(0) += *number;
        });
        std::mem::swap(&mut p1, &mut p2);

        p2.clear();
    }

    p1.iter().for_each(|(k, n)| k.chars().into_iter().for_each(|c| *letters.entry(c).or_insert(0) += n));
    letters.values_mut().for_each(|v| *v /= 2);

    (letters.values().max().unwrap() - letters.values().min().unwrap()) as i32
}

pub fn part_two(filename: &str) -> i64 {
    let lines = read_lines(filename);

    let mut pairs:HashMap<String,i64> = HashMap::new();
    let mut rules = HashMap::new();
    let mut letters = HashMap::new();
    for (i, line) in lines.enumerate() {
        if i == 0 {
            let l0 = line.unwrap().chars().into_iter().collect::<Vec<char>>();
            l0[0..].iter().zip(l0[1..].iter())
                .for_each(|p| *pairs.entry(format!("{}{}", p.0, p.1)).or_insert(0) += 1);
            *letters.entry(l0[0]).or_insert(0) += 1;
            *letters.entry(*l0.last().unwrap()).or_insert(0) += 1;
        } else if i != 1 {
            let rule = line.unwrap().split(" -> ")
                .into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
            let key = rule[0].chars().into_iter().collect::<Vec<char>>();
            let insert_char = rule[1].chars().next().unwrap();

            rules.insert(key.iter().collect::<String>(),
                         ([key[0], insert_char].iter().collect::<String>(),
                          [insert_char, key[1]].iter().collect::<String>()));
        }
    }

    let mut pairs2 = HashMap::new();

    let mut p1 = &mut pairs;
    let mut p2 = &mut pairs2;
    for _ in 0..40 {
        p1.iter().for_each(|(key, number)| {
            let new_keys = rules.get(key).unwrap();
            *p2.entry(new_keys.0.clone()).or_insert(0) += *number;
            *p2.entry(new_keys.1.clone()).or_insert(0) += *number;
        });
        std::mem::swap(&mut p1, &mut p2);

        p2.clear();
    }

    p1.iter().for_each(|(k, n)| k.chars().into_iter().for_each(|c| *letters.entry(c).or_insert(0) += n));
    letters.values_mut().for_each(|v| *v /= 2);

    (letters.values().max().unwrap() - letters.values().min().unwrap()) as i64
}