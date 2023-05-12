// ========================= Challenge Logic ============================
type Bit = char;

#[derive(Debug)]
enum Type {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    Literal = 4,
    GreaterThan = 5,
    LessThan = 6,
    Equal = 7
}

pub fn parse_input(input: String) -> Vec<Bit> {
    input
        .chars()
        .map(|c| c.to_digit(16).expect("Expected base-16 digit"))
        .map(|hex|format!("{:04b}", hex))
        .fold(String::new(), |acc, elem| acc + &elem)
        .chars()
        .collect()
}

pub fn to_digit(v: &[Bit]) -> i64 {
    i64::from_str_radix(&v.iter().collect::<String>(), 2).expect("Could not parse bitvector as integer!")
}

pub fn parse_packet(packet: &[Bit], idx: &mut usize, part1: bool) -> i64 {

    let p_version = to_digit(&packet[*idx..*idx+3]);
    *idx += 3;

    let p_type = match to_digit(&packet[*idx..*idx+3]) {
        0 => Type::Sum,
        1 => Type::Product,
        2 => Type::Minimum,
        3 => Type::Maximum,
        4 => Type::Literal,
        5 => Type::GreaterThan,
        6 => Type::LessThan,
        7 => Type::Equal,
        _ => panic!("Not possible")
    };
    *idx += 3;

    // println!("version: {}, type: {:?}", p_version, p_type);

    match p_type {
        Type::Literal => {
            let mut lit = 0;
            let mut finished = false;
            while !finished {
                // Check if this is the last packet
                finished = to_digit(&packet[*idx..*idx+1]) == 0;
                *idx += 1;

                // Add next 4 bits
                lit = (lit << 4) + to_digit(&packet[*idx..*idx+4]);
                *idx += 4;
            }

            if part1 {
                p_version
            } else {
                lit
            }
        },
        // In case it's an operator
        _ => {
            // Get length type
            let l_type = to_digit(&packet[*idx..*idx+1]);
            *idx += 1;

            // first deal with type 0
            if l_type == 0 {
                let mut total_length = to_digit(&packet[*idx..*idx+15]) as usize;
                *idx += 15;

                let mut lit_v = Vec::new();
                while total_length != 0 {
                    // Store current index
                    let temp_idx = *idx;

                    // add next value to lit vector
                    lit_v.push(parse_packet(&packet, idx, part1));

                    // Subtract what we've read from the total length
                    total_length -= *idx - temp_idx;
                }
                if part1 {
                    p_version + lit_v.iter().sum::<i64>()
                } else {
                    // probably something with lit_v
                    match p_type {
                        Type::Sum => lit_v.iter().sum::<i64>(),
                        Type::Product => lit_v.iter().product::<i64>(),
                        Type::Minimum => *lit_v.iter().min().expect("Expected to find minimum"),
                        Type::Maximum => *lit_v.iter().max().expect("Expected to find minimum"),
                        Type::Literal => panic!("Not possible"),
                        Type::GreaterThan => if lit_v[0] > lit_v[1] { 1 } else { 0 },
                        Type::LessThan => if lit_v[0] < lit_v[1] { 1 } else { 0 },
                        Type::Equal => if lit_v[0] == lit_v[1] { 1 } else { 0 },
                    }
                }
            } else { // then deal with type 1
                let sub_packets = to_digit(&packet[*idx..*idx+11]) as usize;
                *idx += 11;

                let mut lit_v = Vec::new();
                for _ in 0..sub_packets {
                    // Parse subpacket and add value to lit vector
                    lit_v.push(parse_packet(&packet, idx, part1));
                }

                if part1 {
                    p_version + lit_v.iter().sum::<i64>()
                } else {
                    // probably something with lit_v
                    match p_type {
                        Type::Sum => lit_v.iter().sum::<i64>(),
                        Type::Product => lit_v.iter().product::<i64>(),
                        Type::Minimum => *lit_v.iter().min().expect("Expected to find minimum"),
                        Type::Maximum => *lit_v.iter().max().expect("Expected to find minimum"),
                        Type::Literal => panic!("Not possible"),
                        Type::GreaterThan => if lit_v[0] > lit_v[1] { 1 } else { 0 },
                        Type::LessThan => if lit_v[0] < lit_v[1] { 1 } else { 0 },
                        Type::Equal => if lit_v[0] == lit_v[1] { 1 } else { 0 },
                    }
                }
            }
        }
    }
}

pub fn part1(packet: &[Bit]) -> String {
    // println!("{:?}", packet);
    
    let mut idx = 0;
    let val = parse_packet(packet, &mut idx, true);

    format!("{}", val)
}

pub fn part2(packet: &[Bit]) -> String {
    let mut idx = 0;
    let val = parse_packet(packet, &mut idx, false);

    format!("{}", val)
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    let sample = parse_input(read_file("sample"));
    let input = parse_input(read_file("input"));
    formatted_print("A", part1(&input));
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