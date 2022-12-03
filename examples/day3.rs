#![feature(iter_array_chunks)]
use std::collections::HashSet;

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => u32::from(c) - u32::from('a') + 1,
        'A'..='Z' => u32::from(c) - u32::from('A') + 27,
        _ => todo!(),
    }
}

fn main() {
    //let s = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
    //    let s = "vJrwpWtwJgWrhcsFMMfFFhFp
    //jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    //PmmdzqPrVvPwwTWBwg
    //wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    //ttgJtRGJQctTZtZT
    //CrZsJsPPZsGzwwsLwLmpwMDw";
    let s = include_str!("../input_data/day3.txt");
    let s = s.split("\n").filter(|s| s.len() != 0);

    println!("Part 1:");
    println!(
        "Sum: {}",
        s.clone()
            .map(|s| {
                let s1: HashSet<char> = s[0..(s.len() / 2)].chars().collect();
                let s2: HashSet<char> = s[(s.len() / 2)..].chars().collect();

                let diff = s1.intersection(&s2);
                diff.map(|c| priority(*c)).sum::<u32>()
            })
            .sum::<u32>()
    );

    println!("Part 2:");
    println!(
        "Sum: {}",
        s.array_chunks::<3>()
            .map(|[a, b, c]| {
                let s1: HashSet<char> = a.chars().collect();
                let s2: HashSet<char> = b.chars().collect();
                let s3: HashSet<char> = c.chars().collect();

                let i1: HashSet<char> = s1.intersection(&s2).cloned().collect();
                let diff = i1.intersection(&s3);
                diff.map(|c| priority(*c)).sum::<u32>()
            })
            .sum::<u32>()
    );
}
