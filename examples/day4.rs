fn is_in(a: (i32, i32), b: (i32, i32)) -> bool {
    a.0 <= b.0 && b.1 <= a.1
}

fn point_in(a: i32, b: (i32, i32)) -> bool {
    b.0 <= a && a <= b.1
}

fn overlap(a: (i32, i32), b: (i32, i32)) -> bool {
    point_in(a.0, b) || point_in(a.1, b) || point_in(b.0, a)
}

fn main() {
    let s = include_str!("../input_data/day4.txt");
    let s = s.split("\n").filter(|s| s.len() != 0);

    println!(
        "part 1: {}",
        s.clone().map(|s| {
            let raw: Vec<_> = s
                .split(",")
                .map(|s| s.split("-").map(|s| s.parse::<i32>().unwrap()))
                .flatten()
                .collect();
            let a = (raw[0], raw[1]);
            let b = (raw[2], raw[3]);
            (is_in(a, b) || is_in(b, a)) as i32
        })
        .sum::<i32>()
    );

    println!(
        "part 2: {}",
        s.map(|s| {
            let raw: Vec<_> = s
                .split(",")
                .map(|s| s.split("-").map(|s| s.parse::<i32>().unwrap()))
                .flatten()
                .collect();
            let a = (raw[0], raw[1]);
            let b = (raw[2], raw[3]);
            overlap(a,b) as i32
        })
        .sum::<i32>()
    );

}
