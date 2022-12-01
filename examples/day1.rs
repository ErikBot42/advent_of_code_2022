fn main() {
    let s = include_str!("../input_data/day1.txt");

    // part 1
    let a = s
        .split("\n\n")
        .map(|s| {
            s.split("\n")
                .map(|a| a.parse::<u32>().unwrap_or(0))
                .sum::<u32>()
        })
        .max()
        .unwrap();
    println!("max (part 1): {}", a);

    // part 2
    let mut a = s
        .split("\n\n")
        .map(|s| {
            s.split("\n")
                .map(|a| a.parse::<u32>().unwrap_or(0))
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    a.sort();
    println!(
        "top 3 sum (part 2): {}",
        a[(a.len() - 3)..(a.len())].iter().sum::<u32>()
    );
}
