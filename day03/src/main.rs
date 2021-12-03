// ========================= Challenge Logic ============================

pub fn most_common_bit_at_idx(v: &Vec<u64>, i: usize) -> u64 {
    if v.iter().filter(|&c| c & (1 << i) > 0).count() * 2 >= v.len() {
        1
    } else {
        0
    }
}

pub fn part1(input: String) -> String {
    // Parse codes from input
    let codes = input.lines().map(|l| u64::from_str_radix(l, 2).expect("Could not parse code")).collect::<Vec<u64>>();
    // Get number of bits per input line
    let code_len = input.lines().next().unwrap().len();

    // Compute the gamma and epsilon values
    let (gamma, epsilon) = (0..code_len).rev().map(|shift| {
        // if the most common bit == 1, return (2^shift, 0)
        if most_common_bit_at_idx(&codes, shift) == 1 {
            (1 << shift, 0)
        } else { // else return (0, 2^shift)
            (0, 1 << shift)
        }
    }).fold( // Take the 2-powers from the map stage, and add them up to get gamma and epsilon
        (0, 0), 
        |(acc_gamma, acc_epsilon), (elem_gamma, elem_epsilon)| (acc_gamma + elem_gamma, acc_epsilon + elem_epsilon)
    );

    // Return gamma and epsilon
    format!("{}", gamma * epsilon)
}

pub fn part2(input: String) -> String {
    // Parse codes from input
    let codes = input.lines().map(|l| u64::from_str_radix(l, 2).expect("Could not parse code")).collect::<Vec<u64>>();
    // Get number of bits per input line
    let code_len = input.lines().next().unwrap().len();

    // Make clone of codes for ox and co2 since we need to remove values
    let mut ox_candidates = codes.clone();
    let mut co2_candidates = codes.clone();

    (0..code_len).rev().for_each(|shift| {
        // If there are more than 1 ox candidates
        if ox_candidates.len() > 1 {
            // Check for most common bit at position `shift`
            if most_common_bit_at_idx(&ox_candidates, shift) == 1 {
                // if most common bit == 1, filter out elements that don't have a 1 at position `shift`
                ox_candidates = ox_candidates.drain(..).filter(|&c| (c & (1 << shift)) > 0).collect();
            } else {
                // if most common bit == 0, filter out elements that don't have a 0 at position `shift`
                ox_candidates = ox_candidates.drain(..).filter(|&c| (c & (1 << shift)) == 0).collect();
            }
        }

        // If there are more than 1 co2 candidates
        if co2_candidates.len() > 1 {
            // Check for most common bit at position `shift`
            if most_common_bit_at_idx(&co2_candidates, shift) == 1 {
                // if most common bit == 1, filter out elements that don't have a 0 at position `shift`
                co2_candidates = co2_candidates.drain(..).filter(|&c| (c & (1 << shift)) == 0).collect();
            } else {
                // if most common bit == 0, filter out elements that don't have a 1 at position `shift`
                co2_candidates = co2_candidates.drain(..).filter(|&c| (c & (1 << shift)) > 0).collect();
            }
        }
    });

    // Take the oxygen and co2 values, and return the multiplication of them
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