use std::cmp::Ordering;
use std::io;
use std::{fs::File, io::Read};

fn read_input() -> io::Result<String> {
    let mut file = File::open("input.in")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn part1(contents: String) -> io::Result<()> {
    let mut safe_reports = 0;
    for line in contents.lines() {
        let levels = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut safe = true;
        let mut trend: &str = "";
        for i in 0..levels.len() - 1 {
            let state = match levels[i].cmp(&levels[i + 1]) {
                Ordering::Less => "increasing",
                Ordering::Greater => "decreasing",
                Ordering::Equal => {
                    safe = false;
                    break;
                }
            };

            if i == 0 {
                trend = state;
            } else if trend != state {
                safe = false;
                break;
            }

            let diff = (levels[i] - levels[i + 1]).abs();
            if diff > 3 {
                safe = false;
                break;
            }
        }

        if safe {
            safe_reports += 1;
        }
    }

    println!("Part 1: Safe reports: {}", safe_reports);
    Ok(())
}

fn is_safe(levels: &[i32]) -> Option<usize> {
    let mut trend: &str = "";

    for i in 0..levels.len() - 1 {
        let state = match levels[i].cmp(&levels[i + 1]) {
            Ordering::Less => "increasing",
            Ordering::Greater => "decreasing",
            Ordering::Equal => return Some(i),
        };

        if i == 0 {
            trend = state;
        } else if trend != state {
            return Some(i);
        }

        let diff = (levels[i] - levels[i + 1]).abs();
        if diff > 3 {
            return Some(i);
        }
    }

    None
}

fn try_fix_levels(levels: &[i32], index: usize) -> bool {
    let mut corrected_levels = levels.to_vec();

    corrected_levels.remove(index);
    if is_safe(&corrected_levels).is_none() {
        return true;
    }

    if index + 1 < levels.len() {
        let mut corrected_levels = levels.to_vec();
        corrected_levels.remove(index + 1);
        if is_safe(&corrected_levels).is_none() {
            return true;
        }
    }

    if index > 0 {
        let mut corrected_levels = levels.to_vec();
        corrected_levels.remove(index - 1);
        if is_safe(&corrected_levels).is_none() {
            return true;
        }
    }

    false
}

fn part2(contents: String) -> io::Result<()> {
    let mut safe_reports = 0;
    for line in contents.lines() {
        let levels = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let issue_index = is_safe(&levels);
        if let Some(index) = issue_index {
            if !try_fix_levels(&levels, index) {
                continue;
            }
        }

        safe_reports += 1;
    }

    println!("Part 2: Safe reports: {}", safe_reports);
    Ok(())
}

fn main() -> io::Result<()> {
    println!("Day 2: Advent of Code 2024");
    println!("--------------------------");

    let contents = read_input()?;

    part1(contents.clone())?;
    part2(contents.clone())?;

    Ok(())
}
