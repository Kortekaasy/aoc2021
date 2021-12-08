// ========================= Challenge Logic ============================
pub fn parse_input(input: String) -> Vec<i32> {
    input
        .split(",")
        .map(|v| 
            v.parse::<i32>()
            .expect(format!("Could not parse input \"{}\" as i32", v).as_str())
        ).collect()
}

pub fn part1(input: &Vec<i32>) -> String {
    let min = *input.iter().min().expect("Expected to find min");
    let max = *input.iter().max().expect("Expected to find max");

    let min_y = (min..max).map(|y|
        input.iter().map(|x| (y-x).abs()).sum::<i32>()
    ).min().expect("Expected to find minimum y values");

    format!("{}", min_y)
}

pub fn cost(c: i32) -> i32 {
    // assert!(c >= 0, "Cost func doesnt work for negative numbers");
    (c * (c + 1)) / 2
}

pub fn part2(input: &Vec<i32>) -> String {
    let min = *input.iter().min().expect("Expected to find min");
    let max = *input.iter().max().expect("Expected to find max");

    let min_y = (min..max).map(|y|
        input.iter().map(|x| (y-x).abs()).map(cost).sum::<i32>()
    ).min().expect("Expected to find minimum y values");

    format!("{}", min_y)
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    let sample = parse_input(read_file("sample"));
    let input = parse_input(read_file("input"));

    assert_eq!(part1(&sample), "37");
    formatted_print("A", part1(&input));

    assert_eq!(part2(&sample), "168");
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