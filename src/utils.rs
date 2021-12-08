use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};

pub fn read_lines(filename:&str) -> Lines<BufReader<File>>{
    BufReader::new(File::open(filename).expect("unable to open file")).lines()
}

pub trait IntoTuple{
    fn into_tuple(self, needle:&str) -> Vec<(String, i32)>;
}

impl IntoTuple for Lines<BufReader<File>> {
    fn into_tuple(self, needle:&str) -> Vec<(String, i32)> {
        self.map(|l|
            (l.as_ref().expect("unable to read line")
                 .split(needle).nth(0).expect("no object").to_string(),
             l.as_ref().expect("unable to read line")
                 .split(needle).nth(1).expect("no object")
                 .parse::<i32>().expect("unable to parse")))
            .collect::<Vec<(String, i32)>>()
    }
}