use std::collections::HashSet;
use std::iter::FromIterator;

// ========================= Challenge Logic ============================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
    z: i32
}

impl Point {
    fn new() -> Point {
        Point {
            x: 0, y: 0, z: 0,
        }
    }

    fn from(x: i32, y: i32, z: i32) -> Point {
        Point {
            x, y, z
        }
    }
}

#[derive(Clone)]
pub struct Report {
    points: HashSet<Point>
}

impl Report {
    fn new() -> Report {
        Report {
            points: HashSet::new()
        }
    }

    fn corner(&self) -> Point {
        panic!("Unimplemented")
    }

    fn points_relative_to(&self, point: Point) -> Report {
        panic!("Unimplemented")
    }

    fn rotate_x(&self) -> Report {
        panic!("Unimplemented")
    }

    fn rotate_y(&self) -> Report {
        panic!("Unimplemented")
    }

    fn rotate_z(&self) -> Report {
        panic!("Unimplemented")
    }

    fn rotations(&self) -> Vec<Report> {
        let mut rotations = Vec::new();

        
        rotations.push(self.clone());
        for _ in 0..3 {
            rotations.push(rotations[rotations.len()-1].rotate_x());
        }

        rotations.push(rotations[0].rotate_y());
        for _ in 0..3 {
            rotations.push(rotations[rotations.len()-1].rotate_x());
        }

        rotations.push(rotations[4].rotate_y());
        for _ in 0..3 {
            rotations.push(rotations[rotations.len()-1].rotate_x());
        }

        rotations.push(rotations[8].rotate_y());
        for _ in 0..3 {
            rotations.push(rotations[rotations.len()-1].rotate_x());
        }

        rotations.push(rotations[0].rotate_z());
        for _ in 0..3 {
            rotations.push(rotations[rotations.len()-1].rotate_x());
        }

        rotations.push(rotations[16].rotate_z().rotate_z());
        for _ in 0..3 {
            rotations.push(rotations[rotations.len()-1].rotate_x());
        }

        rotations
    }
}

impl FromIterator<Point> for Report {
    fn from_iter<I: IntoIterator<Item=Point>>(iter: I) -> Self {
        Report { points: iter.into_iter().collect::<HashSet<Point>>() }
    }
}

pub fn part1(input: String) -> String {
    format!("Unimplemented!")
}

pub fn part2(input: String) -> String {
    format!("Unimplemented!")
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