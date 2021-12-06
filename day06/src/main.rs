// ========================= Challenge Logic ============================

pub struct LanternFish(i8);

pub fn part1(input: String) -> String {
    let mut fish: Vec<LanternFish> = input.split(",").map(|v| v.parse::<i8>().expect("Could not parse as i8")).map(|v| LanternFish(v)).collect();
    for _ in 0..80 {
        for i in 0..fish.len() {
            if fish[i].0 == 0 {
                fish.push(LanternFish(8));
                fish[i].0 = 7;
            } 
            fish[i].0 -= 1;
        }
    }
    format!("{}", fish.len())
}

use std::collections::HashMap;

pub fn part2(input: String) -> String {
    let end_day = 256;

    let mut days: HashMap<i64, u128> = HashMap::new();
    
    input.split(",").for_each(|v| {
        *days.entry(v.parse::<i64>().expect("Could not parse as i64") - 9).or_insert(0) += 1;
    });

    let mut min_key = *days.keys().min().expect("Expected minimum key to be available");

    while (min_key + 9) < end_day {
        let mut k = min_key;

        *days.entry(k + 9).or_insert(0) += *days.entry(k).or_insert(0);
        k += 9;

        while k + 7 < end_day {
            *days.entry(k + 7).or_insert(0) += days[&min_key];
            k += 7;

        }

        min_key += 1;
    }

    format!("{}", days.values().sum::<u128>())
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