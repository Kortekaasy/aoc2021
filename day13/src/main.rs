use std::collections::HashSet;
// ========================= Challenge Logic ============================
#[derive(Clone, Copy, Debug)]
pub enum Fold {
    Horizontal(usize),
    Vertical(usize)
}

type Field = HashSet<(usize, usize)>;

pub fn parse_input(input: String) -> (Field, Vec<Fold>) {
    let mut line_iter = input.lines();

    // Field parsing
    let mut field = Field::new();
    while let Some(l) = line_iter.next() {
        // If we hit the empty line, break out of field parsing
        if l == "" {
            break;
        }
        let (x, y) = l.split_once(",").expect("Expected to split on comma");
        let (x, y) = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());
        field.insert((x, y));
    }

    // Fold parsing
    let mut folds = Vec::new();
    while let Some(l) = line_iter.next() {
        match l.split_once("=").expect("Could not parse fold") {
            ("fold along x", by) => {
                folds.push(Fold::Horizontal(by.parse::<usize>().expect("Could not parse x")))
            },
            ("fold along y", by) => {
                folds.push(Fold::Vertical(by.parse::<usize>().expect("Could not parse y")))
            },
            (_, _) => panic!("Unexpected input {}", l)
        }
    }

    (field, folds)
}

pub fn part1(mut field: Field, mut fold: Vec<Fold>) -> String {

    for f in fold.drain(0..1) {
        match f {
            Fold::Horizontal(at) => {
                field = field.drain()
                .map(|(x, y)| {
                    if x > at {
                        (at - (x - at), y)
                    } else {
                        (x, y)
                    }
                }).collect();
            },
            Fold::Vertical(at) => {
                field = field.drain()
                    .map(|(x, y)| {
                        if y > at {
                            (x, at - (y - at))
                        } else {
                            (x, y)
                        }
                    }).collect();
            }
        }
    }
    format!("{}", field.len())
}

pub fn part2(mut field: Field, mut fold: Vec<Fold>) -> String {
    for f in fold.drain(..) {
        match f {
            Fold::Horizontal(at) => {
                field = field.drain()
                .map(|(x, y)| {
                    if x > at {
                        (at - (x - at), y)
                    } else {
                        (x, y)
                    }
                }).collect();
            },
            Fold::Vertical(at) => {
                field = field.drain()
                    .map(|(x, y)| {
                        if y > at {
                            (x, at - (y - at))
                        } else {
                            (x, y)
                        }
                    }).collect();
            }
        }
    }

    let x_max = field.iter().map(|&(x, _y)| x).max().expect("Expected max value");
    let y_max = field.iter().map(|&(_x, y)| y).max().expect("Expected max value");

    let mut result = String::new();
    for y in 0..y_max + 1 {
        for x in 0..x_max + 1 {
            if field.contains(&(x, y)) {
                result += "\u{25A0}";
            } else {
                result += " ";
            }
        }
        result += "\n";
    }
    
    result
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    let (s_field, s_folds) = parse_input(read_file("sample"));
    let (i_field, i_folds) = parse_input(read_file("input"));
    
    assert_eq!(part1(s_field.clone(), s_folds.clone()), "17");
    formatted_print("A", part1(i_field.clone(), i_folds.clone()));
    
    // assert_eq!(part2(s_field, s_folds), "Unimplemented!");
    formatted_print("B", part2(i_field, i_folds));
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