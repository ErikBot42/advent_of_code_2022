use std::collections::HashSet;
fn main() {
    let s = include_str!("../input_data/day6.txt");
    [4, 14].into_iter().for_each(|window| {
        println!(
            "{window}: {:?}",
            s.chars()
                .collect::<Vec<_>>()
                .windows(window)
                .enumerate()
                .find_map(|(i, x)| {
                    (x.into_iter()
                        .cloned()
                        .collect::<std::collections::HashSet<char>>()
                        .len()
                        == window)
                        .then_some(i + window)
                })
        )
    });
}
