use std::{ops::Range, collections::HashSet};

// ========================= Challenge Logic ============================
pub fn parse_input(input: String) -> (Range<i64>, Range<i64>) {
    let splitted = input.split(" ").collect::<Vec<&str>>();
    let (x_range, y_range) = (splitted[2], splitted[3]);

    let x_range = x_range.chars().skip(2).take_while(|&c| c != ',').collect::<String>();
    let y_range = y_range.chars().skip(2).collect::<String>();

    let (x_start, x_end) = x_range.split_once("..").expect("Expected x range");
    let (y_start, y_end) = y_range.split_once("..").expect("Expected y range");

    let (x_start, x_end) = (x_start.parse::<i64>().expect("Expected to parse x_start"), x_end.parse::<i64>().expect("Expected to parse x_end"));
    let (y_start, y_end) = (y_start.parse::<i64>().expect("Expected to parse y_start"), y_end.parse::<i64>().expect("Expected to parse y_end"));

    (x_start..x_end+1, y_start..y_end+1)
}

pub fn part1(input: &(Range<i64>, Range<i64>)) -> String {
    let (x_range, y_range) = input;
    // first calculate max steps need to enter x range
    // let mut max_i = -1;
    // fn calc_x(x: i64, i: i64) -> i64 {
    //     (0..i+1).map(|a| if a < x {x - a} else {0}).sum::<i64>()
    // }
    // for x in 1..x_range.end {
    //     let mut i = 0;
    //     while calc_x(x, i) != calc_x(x, i + 1) {
    //         i += 1;
    //     }
    //     if x_range.contains(&calc_x(x, i)) {
    //         max_i = max_i.max(i);
    //     }
    // }
    // println!("max_i: {}", max_i);

    let mut y_candidates = Vec::new();
    let mut y = y_range.start;

    loop {
        // if (0..max_i+1).map(|a| y - a).sum::<i64>() > y_range.end {
        if y > 600 {
            break;
        }
        let mut i = 0;
        let mut cand_y = (0..i+1).map(|a| y - a).sum::<i64>();
        while cand_y >= y_range.start {
            if y_range.contains(&cand_y) {
                y_candidates.push((y, i));
            }
            i+= 1;
            cand_y = (0..i+1).map(|a|y-a).sum::<i64>();
        }
        y += 1;
    }

    let mut xy_candidates = Vec::new();
    for (y, j) in y_candidates {
        for x in 1..x_range.end {
            let cand_x = (0..j+1).map(|a| if a < x {x - a} else {0}).sum::<i64>();
                if x_range.contains(&cand_x) {
                    xy_candidates.push((x, y))
                }
        }
    }

    // Get the trajectory with the highest y
    let y_traject = *xy_candidates.iter().map(|(_x, y)| y).max().expect("Expected max y");let mut i = 0;
    let mut state = 0;

    let mut res = i64::MIN;

    while state < 2 {
        let y_i = (0..i+1).map(|a| y_traject - a).sum::<i64>();
        if y_range.contains(&y_i) {
            state += 1;
        } else if state == 1 {
            state += 1;
        }
        if state < 2 {
            res = res.max(y_i);
        }
        i += 1;
    }


    format!("{}", res)
}

pub fn part2(input: &(Range<i64>, Range<i64>)) -> String {
    let (x_range, y_range) = input;

    let mut y_candidates = Vec::new();
    let mut y = y_range.start;

    loop {
        // if (0..max_i+1).map(|a| y - a).sum::<i64>() > y_range.end {
        if y > 600 {
            break;
        }
        let mut i = 0;
        let mut cand_y = (0..i+1).map(|a| y - a).sum::<i64>();
        while cand_y >= y_range.start {
            if y_range.contains(&cand_y) {
                y_candidates.push((y, i));
            }
            i+= 1;
            cand_y = (0..i+1).map(|a|y-a).sum::<i64>();
        }
        y += 1;
    }
    let mut xy_candidates = Vec::new();
    for (y, j) in y_candidates {
        for x in 1..x_range.end {
            let cand_x = (0..j+1).map(|a| if a < x {x - a} else {0}).sum::<i64>();
                if x_range.contains(&cand_x) {
                    xy_candidates.push((x, y))
                }
        }
    }

    format!("{}", xy_candidates.into_iter().collect::<HashSet<(i64, i64)>>().len())
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    let sample = parse_input(read_file("sample"));
    let input = parse_input(read_file("input"));

    assert_eq!(part1(&sample), "45");
    formatted_print("A", part1(&input));

    assert_eq!(part2(&sample), "112");
    formatted_print("B", part2(&input));
}

pub fn read_file(file_name: &str) -> String {
    return std::fs::read_to_string(file_name).expect(format!("File {} not found", file_name).as_str());
}

fn formatted_print(part: &str, output: String) {
    println!("==================== Part {} ======================", part);
    for l in output.lines() {
        println!("| {:^46} |", l);
    }
    println!("==================================================");
}