// ========================= Challenge Logic ============================

pub fn part1(input: String) -> String {
    let (mut h, mut d) = (0, 0);
    for l in input.lines() {
        if let Some((instr, val)) = l.split_once(" ") {
            let by = val.parse::<i64>().expect("Could not convert val to i64");
            match instr {
                "forward" => h += by,
                "down" => d += by,
                "up" => d -= by,
                _ => panic!("Unknown instruction \"{} {}\"", instr, val)
            }
        }
    }
    format!("{}", h * d)
}

pub fn part2(input: String) -> String {
    let (mut h, mut d, mut aim) = (0, 0, 0);
    for l in input.lines() {
        if let Some((instr, val)) = l.split_once(" ") {
            let by = val.parse::<i64>().expect("Could not convert val to i64");
            match instr {
                "forward" => {
                    h += by;
                    d += aim * by;
                },
                "down" => aim += by,
                "up" => aim -= by,
                _ => panic!("Unknown instruction \"{} {}\"", instr, val)
            }
        }
    }
    format!("{}", h * d)
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