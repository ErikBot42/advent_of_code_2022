fn main() {
    let s = include_str!("../input_data/day1.txt");

    // part 1
    {
        s.split("\n\n")
            .map(|s| {
                s.split("\n")
                    .filter_map(|a| a.parse::<u32>().ok())
                    .sum::<u32>()
            })
            .max()
            .map(|a| println!("max (part 1): {}", a));
    }

    // part 2
    {
        let mut a = s
            .split("\n\n")
            .map(|s| {
                s.split("\n")
                    .filter_map(|a| a.parse::<u32>().ok())
                    .sum::<u32>()
            })
            .collect::<Vec<u32>>();
        a.sort();
        let a = a[(a.len() - 3)..(a.len())].iter().sum::<u32>();
        println!("top 3 sum (part 2): {}", a);
    }
}
