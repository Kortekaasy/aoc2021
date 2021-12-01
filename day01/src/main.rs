// ========================= Challenge Logic ============================

pub fn part1(input: String) -> String {
    // Split input on newline and parse all lines as i64
    let depths: Vec<i64> = input
        .lines()
        .map(|l|
            l.parse::<i64>()
             .expect("Could not parse input as i64")
        )
        .collect();
    
    // Compare every item i+1 with item i, and count how many item i+1's are larger than item i
    let count = depths
        .iter()
        .skip(1)
        .zip(
            depths.iter()
        )
        .filter(|(&next, &current)| 
            next > current
        ).count();

    format!("{}", count)
}

pub fn part2(input: String) -> String {
    // Split input on newline and parse all lines as i64
    let depths: Vec<i64> = input
        .lines()
        .map(|l|
            l.parse::<i64>()
             .expect("Could not parse input as i64")
        )
        .collect();
    
    // Compute the sums of the three-measurement windows
    let sums = depths
        .iter()
        .skip(2)
        .zip(
            depths
                .iter()
                .skip(1)
        )
        .zip(depths.iter())
        .map(|((&nextnext, &next), &current)| 
            nextnext + next + current
        ).collect::<Vec<i64>>();
    
    // Compare every windows with the previous window, and count how many windows are larger than the previous
    let count = sums
        .iter()
        .skip(1)
        .zip(
            sums.iter()
        )
        .filter(|(&next, &current)| 
            next > current
        ).count();

    format!("{}", count)
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    formatted_print("A", part1(read_file("input")));
    formatted_print("B", part2(read_file("input")));
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