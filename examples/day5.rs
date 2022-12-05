#![feature(iter_array_chunks)]
use std::collections::VecDeque;

fn read_top(v: &Vec<VecDeque<char>>) -> String {
    v.iter().map(|v| *v.back().unwrap()).collect()
}

fn transfer(v: &mut Vec<VecDeque<char>>, number: usize, from: usize, to: usize) {
    for _ in 0..number {
        let c = v[from - 1].pop_back().unwrap();
        v[to - 1].push_back(c);
    }
}

fn main() {
    let s = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    let s = include_str!("../input_data/day5.txt");

    let (first, second) = {
        let mut s = s.split("\n\n");
        (s.next().unwrap(), s.next().unwrap())
    };

    let number_of_stacks = first.split("\n").last().unwrap().split_whitespace().count();
    dbg!(number_of_stacks);

    //println!("first:\n{}", first);

    let stacks: Vec<VecDeque<char>> = (0..number_of_stacks).map(|_| VecDeque::new()).collect();
// part 1
    {
        let mut stacks = stacks.clone();
        let mut numbers: Vec<_> = first.split("\n").collect();
        numbers.pop();
        numbers
            .into_iter()
            .map(|x| {
                for (i, c) in x
                    .chars()
                    .skip(1)
                    .step_by(4)
                    .enumerate()
                    .filter(|(_, c)| *c != ' ')
                {
                    stacks[i].push_front(c);
                }
            })
            .for_each(|_| ());

        for x in second.split("\n").filter(|x| *x != "") {
            let mut w = x.split_whitespace();
            let mut n = || w.next().unwrap();
            n();
            let number = n().parse::<usize>().unwrap();
            n();
            let from = n().parse::<usize>().unwrap();
            n();
            let to = n().parse::<usize>().unwrap();
            transfer(&mut stacks, number, from, to);
        }
        dbg!(read_top(&stacks));
    }
}
