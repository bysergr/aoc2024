use std::io;
use std::{fs::File, io::Read};

fn read_input() -> io::Result<String> {
    let mut file = File::open("input.in")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn part1(mut left_list: Vec<i64>, mut right_list: Vec<i64>) -> io::Result<()> {
    left_list.sort();
    right_list.sort();

    let mut distance = 0;
    for _ in 0..left_list.len() {
        distance += (left_list.pop().unwrap() - right_list.pop().unwrap()).abs();
    }

    println!("Part 1: Distance: {}", distance);
    Ok(())
}

fn part2(left_list: Vec<i64>, right_list: Vec<i64>) -> io::Result<()> {
    let mut score = 0;
    for i in left_list {
        score += right_list.iter().filter(|&x| *x == i).count() * i as usize;
    }

    println!("Part 2: Similarity Score: {}", score);
    Ok(())
}

fn main() -> io::Result<()> {
    println!("Day 1: Advent of Code 2024");
    println!("--------------------------");

    let contents = read_input()?;

    let mut left_list: Vec<i64> = Vec::new();
    let mut right_list: Vec<i64> = Vec::new();
    for i in contents.lines() {
        let numbers = i
            .split("   ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        left_list.push(numbers[0]);
        right_list.push(numbers[1]);
    }

    part1(left_list.clone(), right_list.clone())?;
    part2(left_list.clone(), right_list.clone())?;
    Ok(())
}
