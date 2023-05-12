use std::collections::HashMap;
use std::collections::HashSet;

// ========================= Challenge Logic ============================

type Graph = HashMap<String, HashSet<String>>;
type Path = Vec<String>;

pub fn parse_input(input: String) -> Graph {
    let mut graph = Graph::new(); 
    for (from, to) in input.lines().map(|l|l.split_once("-").expect("Expected delim")) {
        graph.entry(String::from(from)).or_insert(HashSet::new()).insert(String::from(to));
        graph.entry(String::from(to)).or_insert(HashSet::new()).insert(String::from(from));
    }
    graph
}

pub fn valid_path_1(s: &String, p: &Path) -> bool {
    s.chars().next().map(|c| c.is_ascii_uppercase()).unwrap_or(false) || (!p.contains(s))
}

pub fn add_to_path(g: &Graph, label: &String, mut path: Path) -> Option<Vec<Path>> {

    path.push(label.clone());

    if label == "end" {
        return Some(vec![path])
    }

    let neighbours = g.get(label)?;
    Some(neighbours.iter()
        .filter(|&n| valid_path_1(n, &path))
        .map(|n| add_to_path(&g, n, path.clone()))
        .filter(|r| r.is_some())
        .map(|r| r.unwrap())
        .fold(Vec::new(), |acc, elem| [acc, elem].concat()))
}

pub fn part1(g: &Graph) -> String {

    let paths = add_to_path(&g, &String::from("start"), Vec::new()).expect("Expected to find at least one path to end");

    format!("{}", paths.len())
}

pub fn is_small_cavern(s: &String) -> bool {
    s.chars().next().map(|c| c.is_ascii_lowercase()).unwrap_or(false)
}

pub fn valid_path_2(s: &String, p: &Path) -> bool {
    if is_small_cavern(s) {
        // if small cavern, check if we visited this cavern already and we haven't visited a small cavern twice yet
        let small_caverns = p.iter().filter(|&cave| is_small_cavern(cave)).cloned().collect::<Path>();
        let small_caverns_set = small_caverns.iter().cloned().collect::<HashSet<String>>();
        
        // if we have visited a small cavern twice, check if we don't have the start/end double
        if small_caverns.len() == small_caverns_set.len() {
            if small_caverns_set.contains(s) && (s == &String::from("start") || s == &String::from("end")) {
                false
            } else {
                true
            }
        } else {
            // if we have visited a small cavern twice, check if we have visited _this_ small cavern before.
            // if so, return false
            if small_caverns_set.contains(s) {
                false
            } else {
                // if we have not visited this small cavern before
                s != &String::from("start") //&& s != &String::from("end")
            }
        }
    } else {
        // Uppercase, so may be added
        true
    }
}

pub fn add_to_path_2(g: &Graph, label: &String, mut path: Path) -> Option<Vec<Path>> {

    path.push(label.clone());

    if label == "end" {
        return Some(vec![path])
    }

    let neighbours = g.get(label)?;
    Some(neighbours.iter()
        .filter(|&n| valid_path_2(n, &path))
        .map(|n| add_to_path_2(&g, n, path.clone()))
        .filter(|r| r.is_some())
        .map(|r| r.unwrap())
        .fold(Vec::new(), |acc, elem| [acc, elem].concat()))
}

pub fn part2(g: &Graph) -> String {
    let paths = add_to_path_2(&g, &String::from("start"), Vec::new()).expect("Expected to find at least one path to end");

    // println!("{:#?}", paths);

    format!("{}", paths.len())
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    let sample = parse_input(read_file("sample"));
    let input = parse_input(read_file("input"));

    assert_eq!(part1(&sample), "10");
    formatted_print("A", part1(&input));

    assert_eq!(part2(&sample), "36");
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