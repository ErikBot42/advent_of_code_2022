// 12155
// 10806
//
// 12276
//
// 9975
fn main() {
    //let s = "A Y\nB X\nC Z";
    let s = include_str!("../input_data/day2.txt");
    let s = s.split("\n").map(|s| s.split(" ").collect::<Vec<&str>>());

    let s: i32 = s
        .filter(|s| s.len() == 2)
        .map(|s| {
            println!("{s:?}");
            if s.len() != 2 {
                0
            } else {
                let rock = 1;
                let paper = 2;
                let sis = 3;

                let shape_score = match s[0] {
                    "A" => match s[1] {
                        "X" => sis,  // d
                        "Y" => rock, // g
                        "Z" => paper,
                        _ => panic!(),
                    },
                    "B" => match s[1] {
                        "X" => rock,  // g
                        "Y" => paper, // d
                        "Z" => sis,
                        _ => panic!(),
                    },
                    "C" => match s[1] {
                        "X" => paper, // i
                        "Y" => sis,   // i
                        "Z" => rock,  // g
                        _ => panic!(),
                    },
                    _ => panic!(),
                };

                let outcome_score = match s[1] {
                    "X" => 0,
                    "Y" => 3,
                    "Z" => 6,
                    _ => panic!(),
                };
                dbg!(shape_score);
                dbg!(outcome_score);

                shape_score + outcome_score

                //let win = 6;
                //let draw = 3;
                //let loss = 0;

                //let shape_score = match s[1] {
                //    "X" => 1,
                //    "Y" => 2,
                //    "Z" => 3,
                //    _ => panic!(),
                //};

                //let outcome_score = match s[1] {
                //    "X" => loss,
                //    "Y" => draw,
                //    "Z" => win,
                //    _=> panic!(),
                //};

                //let outcome_score = match s[0] {
                //    "A" => match s[1] {
                //        "X" => draw, // d
                //        "Y" => win, // g
                //        "Z" => loss,
                //        _ => panic!(),
                //    },
                //    "B" => match s[1] {
                //        "X" => loss, // g
                //        "Y" => draw, // d
                //        "Z" => win,
                //        _ => panic!(),
                //    },
                //    "C" => match s[1] {
                //        "X" => win, // i
                //        "Y" => loss, // i
                //        "Z" => draw, // g
                //        _ => panic!(),
                //    },
                //    _ => panic!(),
                //};

                //shape_score + outcome_score
            }
        })
        .sum();
    dbg!(s);
}
