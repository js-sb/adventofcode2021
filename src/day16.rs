use std::fmt::{Display, Formatter};
use itertools::Itertools;
use crate::utils::read_lines;

#[derive(Debug, Clone)]
struct Packet<'a> {
    version: i32,
    packet_type: i32,
    bit_stream: &'a [bool],
}

impl<'a> Packet<'a> {
    fn new(version: i32, packet_type: i32, bit_stream: &'a [bool]) -> Self {
        Packet { version, packet_type, bit_stream }
    }
}

impl<'a> Display for Packet<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "v:{} ,t:{}\n{}\n", self.version, self.packet_type,
               self.bit_stream.iter()
                   .format_with("", |b, f| f(&format_args!("{}", *b as i32))))
    }
}

fn parse_stream(bit_stream: &[bool]) -> Vec<Packet> {
    let mut indicator = bit_stream;
    let mut packets = vec![];

    let info = read_ids(bit_stream);
    let new_packet = Packet::new(info.0, info.1, info.2);
    packets.push(new_packet);
    let mut sub_package = parse_packet(info);
    packets.append(&mut sub_package.0);
    indicator = sub_package.1;

    packets
}

fn read_ids(bit_stream: &[bool]) -> (i32, i32, &[bool]) {
    print!("[{}{}{}][{}{}{}]",
           bit_stream[0] as i32,
           bit_stream[1] as i32,
           bit_stream[2] as i32,
           bit_stream[3] as i32,
           bit_stream[4] as i32,
           bit_stream[5] as i32);
    (
        bit_stream[0] as i32 * 4 + bit_stream[1] as i32 * 2 + bit_stream[2] as i32,
        bit_stream[3] as i32 * 4 + bit_stream[4] as i32 * 2 + bit_stream[5] as i32,
        &bit_stream[6..]
    )
}

fn parse_packet((packet_version, packet_type, bit_stream): (i32, i32, &[bool])) -> (Vec<Packet>, &[bool]) {
    print!("<");
    let ret =
        match packet_type {
            4 => parse_literal(bit_stream),/* literal value */
            _ => parse_operator(bit_stream),
        };
    print!(">");
    ret
}

fn parse_literal(bit_stream: &[bool]) -> (Vec<Packet>, &[bool]) {
    let mut indicator = bit_stream;

    let mut data = vec![];
    while indicator[0] {
        data.append(&mut indicator[1..5].iter().cloned().collect());
        indicator = &indicator[5..];
    }
    data.append(&mut indicator[1..5].iter().cloned().collect());

    print!("\"{}\"",data.iter()
        .format_with("", |b, f| f(&format_args!("{}", *b as i32))));

    (vec![], &indicator[5..])
}

fn parse_operator(bit_stream: &[bool]) -> (Vec<Packet>, &[bool]) {
    let mut data: Vec<&[bool]> = vec![];
    let mut indicator = bit_stream;
    print!("{{{}}}",bit_stream[0] as i32);
    if bit_stream[0] {
        let n_packages =
            (bit_stream[1] as i32 * 1024 +
                bit_stream[2] as i32 * 512 +
                bit_stream[3] as i32 * 256 +
                bit_stream[4] as i32 * 128 +
                bit_stream[5] as i32 * 64 +
                bit_stream[6] as i32 * 32 +
                bit_stream[7] as i32 * 16 +
                bit_stream[8] as i32 * 8 +
                bit_stream[9] as i32 * 4 +
                bit_stream[10] as i32 * 2 +
                bit_stream[11] as i32) as usize;

        print!("## {:?} || {}:0b{1:b} ##)", n_packages, bit_stream.len());
        print!("({} = {})", bit_stream[1..12].iter()
            .format_with("", |b, f| f(&format_args!("{}", *b as i32))),
            n_packages);
        indicator = &bit_stream[12..];
        for i_packet in 0..n_packages {
            data.push(&indicator[11 * i_packet..11 * (i_packet + 1)])
        }
    } else {
        let bit_size =
            (bit_stream[1] as i32 * 16384 +
                bit_stream[2] as i32 * 8192 +
                bit_stream[3] as i32 * 4096 +
                bit_stream[4] as i32 * 2048 +
                bit_stream[5] as i32 * 1024 +
                bit_stream[6] as i32 * 512 +
                bit_stream[7] as i32 * 256 +
                bit_stream[8] as i32 * 128 +
                bit_stream[9] as i32 * 64 +
                bit_stream[10] as i32 * 32 +
                bit_stream[11] as i32 * 16 +
                bit_stream[12] as i32 * 8 +
                bit_stream[13] as i32 * 4 +
                bit_stream[14] as i32 * 2 +
                bit_stream[15] as i32) as usize;

        print!("({} = {})", bit_stream[1..16].iter()
            .format_with("", |b, f| f(&format_args!("{}", *b as i32))),
            bit_size);

        indicator = &bit_stream[16..];
        while indicator[0] {
            data.push(&indicator[0..bit_size]);
            indicator = &indicator[bit_size..];
        }
    }

    let mut packets = vec![];
    for p in data {
        packets.append(&mut parse_stream(p));
    }

    (packets, indicator)
}

pub fn part_one(filename: &str) -> i32 {
    let lines = read_lines(filename);
    let bit_stream = lines.into_iter().nth(0).unwrap().unwrap().chars()
        .fold(vec![], |mut bit_stream, hex_digit| {
            let num = i32::from_str_radix(&hex_digit.to_string(), 16).unwrap();
            bit_stream.push(num & 0b1000 != 0);
            bit_stream.push(num & 0b0100 != 0);
            bit_stream.push(num & 0b0010 != 0);
            bit_stream.push(num & 0b0001 != 0);
            bit_stream
        });

    let mut indicator: &[bool] = &bit_stream;

    println!("{}", bit_stream.iter()
        .format_with("", |b, f| f(&format_args!("{}", *b as i32))));

    let mut version_sum = 0;
    let packet = parse_stream(indicator);
    println!();
    // for p in packet {
    //     println!("{}", p);
    // }

    version_sum
}

pub fn part_two(filename: &str) -> i32 {
    let lines = read_lines(filename);
    0
}