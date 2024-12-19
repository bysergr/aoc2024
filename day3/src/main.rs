use regex::Regex;
use std::io;
use std::{fs::File, io::Read};

fn read_input() -> io::Result<String> {
    let mut file = File::open("input.in")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn extract_mul_operation(commands: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut result = 0;

    let caps = re
        .captures_iter(commands)
        .map(|caps| {
            let (_, [a, b]) = caps.extract();

            let op1 = a.parse::<i32>().unwrap();
            let op2 = b.parse::<i32>().unwrap();

            (op1, op2)
        })
        .collect::<Vec<(i32, i32)>>();

    for (a, b) in caps {
        result += a * b;
    }

    result
}

fn part1(contents: &str) {
    println!("Part 1: Result: {}", extract_mul_operation(contents));
}

fn part2(contents: &str) {
    let mut result = 0;

    let lines = contents.split("don't()").collect::<Vec<&str>>();

    for (i, line) in lines.into_iter().enumerate() {
        if i == 0 {
            result += extract_mul_operation(line);
            continue;
        }

        let slice = line.split_once("do()");

        if slice.is_none() {
            continue;
        }

        let (_, correct_commands) = slice.unwrap();

        result += extract_mul_operation(correct_commands);
    }

    println!("Part 2: Result: {}", result);
}

fn main() -> io::Result<()> {
    println!("Day 3: Advent of Code 2024");
    println!("--------------------------");

    let contents = read_input()?;

    part1(&contents.clone());
    part2(&contents.clone());

    Ok(())
}
