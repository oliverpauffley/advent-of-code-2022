use array_tool::vec::Intersect;
use std::str::FromStr;

fn main() {
    let file = include_str!("input");

    // part 1
    let sum = file
        .lines()
        .map(|line| {
            line.parse::<Rucksack>()
                .expect("could not parse line as rucksack")
        })
        .fold(0, |acc, sack| acc + char_to_priority(&sack.find_matching()));

    println!("sum: {sum:?}");

    // part 2
    let sum = file
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
        .chunks(3)
        .map(|lines| {
            *lines[0]
                .intersect(lines[1].intersect(lines[2].clone()))
                .first()
                .expect("could not find matching value")
        })
        .fold(0, |acc, x| acc + char_to_priority(&x));

    println!("sum: {sum:?}");
}

#[derive(Debug)]
struct Rucksack {
    compartment_1: String,
    compartment_2: String,
}

impl Rucksack {
    fn find_matching(&self) -> char {
        *self
            .compartment_1
            .chars()
            .collect::<Vec<char>>()
            .intersect(self.compartment_2.chars().collect::<Vec<char>>())
            .first()
            .expect("could not find matching element in lists")
    }
}

impl FromStr for Rucksack {
    type Err = color_eyre::Report;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        // split the str in half
        let (first, second) = value.split_at(value.len() / 2);
        Ok(Self {
            compartment_1: first.to_string(),
            compartment_2: second.to_string(),
        })
    }
}

fn char_to_priority(c: &char) -> u32 {
    match *c as u8 {
        b'a'..=b'z' => 1 + (*c as u8 - b'a') as u32,
        b'A'..=b'Z' => 27 + (*c as u8 - b'A') as u32,
        _ => unreachable!(),
    }
}
