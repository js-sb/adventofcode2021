use crate::utils::read_lines;

pub fn part_one(filename: &str) -> i32 {
    let lines = read_lines(filename);
    let mut score = 0;
    for line in lines {
        let mut bracket_buffer: Vec<char> = vec![];
        for ch in line.expect("unable to read line").chars() {
            match ch {
                '{' | '<' | '[' | '(' => bracket_buffer.push(ch),
                '}' | '>' | ']' | ')' => {
                    match (bracket_buffer.pop().expect("no entry"), ch) {
                        ('{', '}') => (),
                        ('[', ']') => (),
                        ('<', '>') => (),
                        ('(', ')') => (),
                        _ => {
                            score += match ch {
                                '}' => 1197,
                                '>' => 25137,
                                ']' => 57,
                                ')' => 3,
                                _ => panic!()
                            };
                            break;
                        }
                    }
                }
                _ => (),
            }
        }
    }
    score
}

pub fn part_two(filename: &str) -> i64 {
    let lines = read_lines(filename);
    let mut scores: Vec<i64> = vec![];
    'outer: for line in lines {
        let mut bracket_buffer: Vec<char> = vec![];
        for ch in line.expect("unable to read line").chars() {
            match ch {
                '{' | '<' | '[' | '(' => bracket_buffer.push(ch),
                '}' | '>' | ']' | ')' => {
                    match (bracket_buffer.pop().expect("no entry"), ch) {
                        ('{', '}') => (),
                        ('[', ']') => (),
                        ('<', '>') => (),
                        ('(', ')') => (),
                        _ => {
                            continue 'outer;
                        }
                    }
                }
                _ => (),
            }
        }
        scores.push(bracket_buffer.into_iter().rev()
            .fold(0, |score, ch| score * 5
                + match ch {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0
            }));
    }
    scores.sort();
    scores[scores.len() / 2]
}