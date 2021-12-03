// ========================= Challenge Logic ============================

pub fn part1(input: String) -> String {
    let codes = input.lines().map(|l| i64::from_str_radix(l, 2).expect("Could not parse code")).collect::<Vec<i64>>();
    let code_len = input.lines().next().unwrap().len();

    let (gamma, epsilon) = (0..code_len).map(|shift| {
        if codes.iter().map(|&c| c & (1 << shift)).filter(|&b| b > 0).count() > (codes.len() / 2) {
            (1 << shift, 0)
        } else {
            (0, 1 << shift)
        }
    }).fold((0, 0), |(acc_gamma, acc_epsilon), (elem_gamma, elem_epsilon)| (acc_gamma + elem_gamma, acc_epsilon + elem_epsilon));
    format!("{}", gamma * epsilon)
}

pub fn part2(input: String) -> String {
    let codes = input.lines().map(|l| u64::from_str_radix(l, 2).expect("Could not parse code")).collect::<Vec<u64>>();
    let code_len = input.lines().next().unwrap().len();

    let mut ox_candidates = codes.clone();
    let mut co2_candidates = codes.clone();

    (0..code_len).rev().for_each(|shift| {
        if ox_candidates.len() > 1 {
            if ox_candidates.iter().map(|&c| c & (1 << shift)).filter(|&b| b > 0).count() * 2 >= (ox_candidates.len()) {
                ox_candidates = ox_candidates.drain(..).filter(|&c| (c & (1 << shift)) > 0).collect();
            } else {
                ox_candidates = ox_candidates.drain(..).filter(|&c| (c & (1 << shift)) == 0).collect();
            }
        }

        if co2_candidates.len() > 1 {
            if co2_candidates.iter().map(|&c| c & (1 << shift)).filter(|&b| b > 0).count() * 2 >= co2_candidates.len() {
                co2_candidates = co2_candidates.drain(..).filter(|&c| (c & (1 << shift)) == 0).collect();
            } else {
                co2_candidates = co2_candidates.drain(..).filter(|&c| (c & (1 << shift)) > 0).collect();
            }
        }
    });
    let ox = ox_candidates[0];
    let co2 = co2_candidates[0];
    format!("{}", ox * co2)
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