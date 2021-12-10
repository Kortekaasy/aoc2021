// ========================= Challenge Logic ============================

pub fn part1(input: String) -> String {
    let mut stack: Vec<char> = Vec::new();
    let score = input.lines()
        .map(|l|
            l.chars()
            .map(|c|
                match c {
                    '(' => {
                        stack.push('(');
                        0
                    },
                    '[' => {
                        stack.push('[');
                        0
                    },
                    '{' => {
                        stack.push('{');
                        0
                    },
                    '<' => {
                        stack.push('<');
                        0
                    },
                    ')' => {
                        if let Some(opening) = stack.pop() {
                            if opening != '(' {
                                // println!("Expected {} found \')\'", opening);
                                3
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    },
                    ']' => {
                        if let Some(opening) = stack.pop() {
                            if opening != '[' {
                                // println!("Expected {} found \']\'", opening);
                                57
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    },
                    '}' => {
                        if let Some(opening) = stack.pop() {
                            if opening != '{' {
                                // println!("Expected {} found \'}}\'", opening);
                                1197
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    },
                    '>' => {
                        if let Some(opening) = stack.pop() {
                            if opening != '<' {
                                // println!("Expected {} found \'>\'", opening);
                                25137
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    },
                    _ => panic!("Unexpected character")
                }
            ).filter(|&score| score != 0).next().unwrap_or(0)
        ).sum::<i64>();
    format!("{}", score)
}

pub fn part2(input: String) -> String {
    let mut stack: Vec<char> = Vec::new();
    let mut scores = input.lines()
        .map(|l| {
            stack.clear();
            let score = l.chars()
            .map(|c|
                match c {
                    '(' => {
                        stack.push('(');
                        0
                    },
                    '[' => {
                        stack.push('[');
                        0
                    },
                    '{' => {
                        stack.push('{');
                        0
                    },
                    '<' => {
                        stack.push('<');
                        0
                    },
                    ')' => {
                        if let Some(opening) = stack.pop() {
                            if opening != '(' {
                                // println!("Expected {} found \')\'", opening);
                                3
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    },
                    ']' => {
                        if let Some(opening) = stack.pop() {
                            if opening != '[' {
                                // println!("Expected {} found \']\'", opening);
                                57
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    },
                    '}' => {
                        if let Some(opening) = stack.pop() {
                            if opening != '{' {
                                // println!("Expected {} found \'}}\'", opening);
                                1197
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    },
                    '>' => {
                        if let Some(opening) = stack.pop() {
                            if opening != '<' {
                                // println!("Expected {} found \'>\'", opening);
                                25137
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    },
                    _ => panic!("Unexpected character")
                }
            ).filter(|&score| score != 0).next().unwrap_or(0);
            if score == 0 {
                stack.clone()
            } else {
                Vec::new()
            }
        })
        .filter(|stack_clone| !stack_clone.is_empty())
        .map(|stack_clone| stack_clone.iter().rev().fold(0, |acc, &elem| {
            acc * 5 + match elem {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("Not possible elem: {}", elem)
            }
        }))
        .collect::<Vec<i128>>();
    scores.sort();
    format!("{}", scores[scores.len() / 2])
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