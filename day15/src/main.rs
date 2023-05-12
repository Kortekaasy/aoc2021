use std::collections::{HashMap, BinaryHeap, HashSet};
use itertools::Itertools;
use std::cmp::Ordering;
// ========================= Challenge Logic ============================

type Cave = Vec<Vec<u32>>;

#[derive(Eq, PartialEq)]
pub struct Node {
    pub coord: (usize, usize),
    pub weight: u32
}

impl Node {

    pub fn new(coord: (usize, usize), weight: u32) -> Node {
        Node {
            coord,
            weight
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight).then_with(|| self.coord.cmp(&other.coord))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


pub fn parse_input(input: String) -> Cave {
    input.lines()
        .map(|l| 
            l.chars()
            .map(|c|c.to_digit(10).expect("Could not parse risk"))
            .collect::<Vec<u32>>()
        ).collect()
}

pub fn manhattan(from: &(usize, usize), to: &(usize, usize)) -> u32 {
    let (x0, y0) = from;
    let (x1, y1) = to;

    let dx = if x0 < x1 {
        x1 - x0
    } else {
        x0 - x1
    };

    let dy = if y0 < y1 {
        y1 - y0
    } else {
        y0 - y1
    };

    (dx + dy) as u32
}

pub fn euclidean(from: &(usize, usize), to: &(usize, usize)) -> u32 {
    let (x0, y0) = from;
    let (x1, y1) = to;

    let dx = if x0 < x1 {
        x1 - x0
    } else {
        x0 - x1
    };

    let dy = if y0 < y1 {
        y1 - y0
    } else {
        y0 - y1
    };

    ((dx * dx) as f64 + (dy * dy) as f64).sqrt() as u32
}

pub fn a_star(c: &Cave, source: (usize, usize), goal: (usize, usize)) -> u32 {
    let mut openSet = HashSet::new();
    openSet.insert(source);

    let mut cameFrom: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();
    
    let mut gScore: HashMap<(usize, usize), u32> = HashMap::new();
    gScore.insert(source, 0);

    let mut fScore = BinaryHeap::new();
    fScore.push(Node::new(source, euclidean(&source, &goal)));


    while let Some(current) = fScore.pop() {
        let u = current.coord;
        if !openSet.contains(&u) {
            continue;
        }
        openSet.remove(&u);
        
        if u == goal {
            return gScore[&u];
        }
        
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let v = ((u.0 as i32 + dx) as usize, (u.1 as i32 + dy) as usize);
            if v.0 > goal.0 || v.1 > goal.1 {
                continue;
            }
            let tentative_gScore = gScore[&u] + c[v.1][v.0];
            if tentative_gScore <= *gScore.get(&v).unwrap_or(&u32::MAX) {
                gScore.insert(v, tentative_gScore);
                cameFrom.insert(v, Some(u));
                let heuristic =  manhattan(&v, &goal);
                fScore.push(Node::new(v, tentative_gScore +heuristic));
                if !openSet.contains(&v) {
                    openSet.insert(v);
                }
            }
        }

    }

    gScore[&goal]
}

pub fn print_cave(c: &mut Cave) {
    let (x_max, y_max) = (c[0].len(), c.len());

    for y in 0..y_max {
        for x in 0..x_max {
            print!("{}", c[y][x]);
            
        }
        println!("");
    }
}

pub fn part1(c: &Cave) -> String {
    let (x_max, y_max) = (c[0].len(), c.len());
    format!("{}", a_star(c, (0, 0), (x_max - 1, y_max - 1)))
}

pub fn part2(c: &Cave) -> String {
    let (x_max, y_max) = (c[0].len(), c.len());
    let expanded = (0..y_max * 5).
        map(|y|
            (0..x_max * 5)
                .map(|x| {
                    let down_shift = (y / y_max) as u32;
                    let right_shift = (x / x_max) as u32;
                    ((c[y % y_max][x % x_max] + down_shift + right_shift - 1) % 9) + 1
                }).collect::<Vec<u32>>()
        ).collect::<Cave>();
    
    format!("{}", a_star(&expanded, (0, 0), (x_max * 5 - 1, y_max * 5 - 1)))
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    let sample = parse_input(read_file("sample"));
    let input = parse_input(read_file("input"));

    assert_eq!(part1(&sample), "40");
    formatted_print("A", part1(&input));

    assert_eq!(part2(&sample), "315");
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