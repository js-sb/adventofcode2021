use itertools::Itertools;
use crate::utils::read_lines;

#[derive(Debug, Clone)]
struct Package {
    version: u64,
    packet_type: u64,
    content: Content,
}

impl Package {
    fn new(version: u64, packet_type: u64, content: Content) -> Self {
        Package { version, packet_type, content }
    }
}

#[derive(Debug, Clone)]
struct Content {
    sub_packets: Option<Vec<Package>>,
    number: Option<u64>,
}

impl Content {
    fn new_packets(sub_packets: Vec<Package>) -> Self {
        Content { sub_packets: Some(sub_packets), number: None }
    }
    fn new_number(number: u64) -> Self {
        Content { sub_packets: None, number: Some(number) }
    }
}


fn slice_to_num(slice: &[bool]) -> u64 {
    slice.iter().rev().enumerate()
        .fold(0u64, |n, (i, b)| n + *b as u64 * 2u64.pow(i as u32))
}

fn _format_slice(slice: &[bool]) -> String {
    format!("[{}]", slice.iter().format_with("", |b, f| f(&format_args!("{}", *b as i32))))
}


fn parse_package(bit_stream: &[bool], break_after_n: Option<u64>) -> (Vec<Package>, &[bool]) {
    let mut stream = bit_stream;
    let mut packets: Vec<Package> = vec![];


    for i in 0.. {
        // break condition 0: only 0 digits left in stream
        if stream.is_empty() || !stream.iter().any(|v| *v) {
            break;
        }

        // break condition 1: read specified number of packages
        if let Some(n) = break_after_n {
            if i >= n {
                break;
            }
        }

        // read package header
        let (version, package_type, slice) =
            parse_ids(stream);

        stream = slice; // move ahead in stream

        // read package contents
        let (content, slice) =
            match package_type {
                4 => parse_literal(stream),
                _ => parse_operator(stream),
            };

        packets.push(Package::new(version, package_type, content));

        stream = slice; // move ahead in stream
    }
    (packets, stream)
}

fn parse_ids(bit_stream: &[bool]) -> (u64, u64, &[bool]) {
    // print!("{}{}",
    //        format_slice(&bit_stream[0..3]),
    //        format_slice(&bit_stream[3..6]));

    (slice_to_num(&bit_stream[0..3]), slice_to_num(&bit_stream[3..6]), &bit_stream[6..])
}


fn parse_literal(bit_stream: &[bool]) -> (Content, &[bool]) {
    let mut indicator = bit_stream;

    // print!(" literal ");

    let mut data = vec![];
    while indicator[0] {
        data.append(&mut indicator[1..5].iter().cloned().collect());
        indicator = &indicator[5..];
    }
    data.append(&mut indicator[1..5].iter().cloned().collect());

    // println!("\"{}\" = {}", format_slice(&*data), slice_to_num(&data));

    (Content::new_number(slice_to_num(&*data)), &indicator[5..])
}

fn parse_operator(bit_stream: &[bool]) -> (Content, &[bool]) {
    let mut stream = bit_stream;

    let content;

    if bit_stream[0] { // break on package number

        // print!(" operator 1 ");
        // println!("{}", format_slice(stream));

        let package_count = slice_to_num(&stream[1..12]);
        stream = &stream[12..];

        let (packets, slice) = parse_package(stream, Some(package_count));
        stream = slice;
        content = Content::new_packets(packets);
    } else { // break on bit count number

        // print!(" operator 0 ");
        // println!("{}", format_slice(stream));

        let bit_count = slice_to_num(&stream[1..16]);
        stream = &stream[16..];

        let (packets, _) = parse_package(&stream[..bit_count as usize], None);
        stream = &stream[bit_count as usize..];
        content = Content::new_packets(packets);
    }


    (content, stream)
}

pub fn part_one(filename: &str) -> u64 {
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


    let (packages, _) = parse_package(&bit_stream, None);
    // println!("\n\n{:?}", packages);


    fn sum_versions(ps: Vec<Package>) -> u64 {
        let mut v = 0;
        for p in ps {
            v += p.version;
            if let Some(s) = p.content.sub_packets {
                v += sum_versions(s);
            }
        }
        v
    }

    sum_versions(packages)
}


pub fn part_two(filename: &str) -> u64 {
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


    let (packages, _) = parse_package(&bit_stream, None);
    // println!("\n\n{:?}", packages);


    fn execute_operations(ps: Vec<Package>, op: u64) -> u64 {
        // ID 0  sum packets          - the sum of the values of their sub-packets. If they only have a single sub-packet, their value is the value of the sub-packet.
        // ID 1  product packets      - the result of multiplying together the values of their sub-packets. If they only have a single sub-packet, their value is the value of the sub-packet.
        // ID 2  minimum packets      - the minimum of the values of their sub-packets.
        // ID 3  maximum packets      - the maximum of the values of their sub-packets.
        // ID 5  greater than packets - 1 if the value of the first sub-packet is greater than the value of the second sub-packet; otherwise, their value is 0.
        // ID 6  less than packets    - 1 if the value of the first sub-packet is less than the value of the second sub-packet; otherwise, their value is 0.
        // ID 7  equal to packets     - 1 if the value of the first sub-packet is equal to the value of the second sub-packet; otherwise, their value is 0.

        // println!("{}, {:?}", op, ps);
        match op {
            0 => // sum
                ps.iter()
                    .fold(0u64, |v, p|
                        if let Some(sub_packets) = p.content.sub_packets.clone() {
                            v + execute_operations(sub_packets, p.packet_type)
                        } else {
                            v + p.content.number.unwrap()
                        }),
            1 => // product
                ps.iter()
                    .fold(1u64, |v, p|
                        if let Some(sub_packets) = p.content.sub_packets.clone() {
                            v * execute_operations(sub_packets, p.packet_type)
                        } else {
                            v * p.content.number.unwrap()
                        }),
            2 => // minimum
                ps.iter()
                    .fold(u64::MAX, |v, p|
                        if let Some(sub_packets) = p.content.sub_packets.clone() {
                            v.min(execute_operations(sub_packets, p.packet_type))
                        } else {
                            v.min(p.content.number.unwrap())
                        }),
            3 => // maximum
                ps.iter()
                    .fold(0u64, |v, p|
                        if let Some(sub_packets) = p.content.sub_packets.clone() {
                            v.max(execute_operations(sub_packets, p.packet_type))
                        } else {
                            v.max(p.content.number.unwrap())
                        }),

            v => // comparison
                {
                    let val0 =
                        if let Some(sub_packets) = ps[0].content.sub_packets.clone() {
                            execute_operations(sub_packets, ps[0].packet_type)
                        } else {
                            ps[0].content.number.unwrap()
                        };
                    let val1 =
                        if let Some(sub_packets) = ps[1].content.sub_packets.clone() {
                            execute_operations(sub_packets, ps[1].packet_type)
                        } else {
                            ps[1].content.number.unwrap()
                        };

                    match v {
                        5 => // greater than
                            (val0 > val1) as u64,
                        6 => // smaller than
                            (val0 < val1) as u64,
                        7 => // equal to
                            (val0 == val1) as u64,
                        _ => { panic!("version is {}", v) }
                    }
                }
        }
    }

    execute_operations(packages, 0)
}