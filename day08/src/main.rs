use std::collections::HashMap;
use std::collections::HashSet;
use itertools::Itertools;

// ========================= Challenge Logic ============================

pub struct Line {
    signal: Vec<String>,
    output: Vec<String>
}

pub fn intersect(a: &String, b: &String) -> HashSet<char> {
    a.chars()
        .collect::<HashSet<char>>()
        .intersection(
            &b.chars().collect()
        ).map(|c|
            *c
        ).collect()
}


impl Line {

    pub fn new(signal: Vec<String>, output: Vec<String>) -> Line {
        Line {
            signal,
            output
        }
    }

    pub fn get_mapping_1(&self) -> (HashMap<i32, String>, HashMap<String, i32>) {
        let mut mapping: HashMap<i32, String> = HashMap::new();

        mapping.insert(1, self.signal.iter().filter(|&s|s.len() == 2).next().unwrap().clone());
        mapping.insert(4, self.signal.iter().filter(|&s|s.len() == 4).next().unwrap().clone());
        mapping.insert(7, self.signal.iter().filter(|&s|s.len() == 3).next().unwrap().clone());
        mapping.insert(8, self.signal.iter().filter(|&s|s.len() == 7).next().unwrap().clone());

        let rev_mapping = mapping.clone().into_iter().map(|(k, v)| (v, k)).collect::<HashMap<String, i32>>();

        (mapping, rev_mapping)
    }

    pub fn get_mapping_2(&self) -> (HashMap<i32, String>, HashMap<String, i32>) {
        let mut mapping: HashMap<i32, String> = HashMap::new();

        mapping.insert(1, self.signal.iter().filter(|&s|s.len() == 2).next().unwrap().clone());
        mapping.insert(4, self.signal.iter().filter(|&s|s.len() == 4).next().unwrap().clone());
        mapping.insert(7, self.signal.iter().filter(|&s|s.len() == 3).next().unwrap().clone());
        mapping.insert(8, self.signal.iter().filter(|&s|s.len() == 7).next().unwrap().clone());
        mapping.insert(6, self.signal.iter().filter(|&s|s.len() == 6).filter(|&s|intersect(s, &mapping[&1]).len() == 1).next().unwrap().clone());
        mapping.insert(3, self.signal.iter().filter(|&s|s.len() == 5).filter(|&s|intersect(s, &mapping[&1]).len() == 2).next().unwrap().clone());
        mapping.insert(9, self.signal.iter().filter(|&s|s.len() == 6).filter(|&s|intersect(s, &mapping[&3]).len() == 5).next().unwrap().clone());
        mapping.insert(5, self.signal.iter().filter(|&s|s.len() == 5).filter(|&s|intersect(s, &mapping[&6]).len() == 5).next().unwrap().clone());
        mapping.insert(2, self.signal.iter().filter(|&s|s.len() == 5).filter(|&s|intersect(s, &mapping[&5]).len() == 3).next().unwrap().clone());
        mapping.insert(0, self.signal.iter().filter(|&s|s.len() == 6).filter(|&s|intersect(s, &mapping[&5]).len() == 4).next().unwrap().clone());

        let rev_mapping = mapping.clone().into_iter().map(|(k, v)| (v, k)).collect::<HashMap<String, i32>>();

        (mapping, rev_mapping)
    }

    pub fn get_output_numbers_1(&self) -> Vec<i32> {
        let (_mapping, rev_mapping) = self.get_mapping_1();
        self.output.iter().map(|s| *rev_mapping.get(s).unwrap_or(&-1)).collect()
    }

    pub fn get_output_numbers_2(&self) -> Vec<i32> {
        let (_mapping, rev_mapping) = self.get_mapping_2();
        self.output.iter().map(|s| rev_mapping[s]).collect()
    }

}

pub fn parse_input(input: String) -> Vec<Line> {
    input.lines()
        .map(|l|{
            let (signal_str, output_str) = l.split_once(" | ").expect("Expected pipe separator");
            let signal = signal_str.splitn(10, " ").map(|s| s.chars().sorted().collect()).collect::<Vec<String>>();
            let output = output_str.splitn(4, " ").map(|s| s.chars().sorted().collect()).collect::<Vec<String>>();

            Line::new(signal, output)
    }).collect()
}

pub fn part1(input: &Vec<Line>) -> String {
    let mut digit_counts: HashMap<i32, i32> = HashMap::new();

    input.iter()
        .map(|l| 
            l.get_output_numbers_1()
        ).for_each(|v| 
            v.iter()
            .for_each(|&i|
                *digit_counts.entry(i).or_insert(0) += 1)
        );

    let count = digit_counts[&1] + digit_counts[&4] + digit_counts[&7] + digit_counts[&8];

    format!("{}", count)
}

pub fn part2(input: &Vec<Line>) -> String {

    let count = input.iter()
        .map(|l| 
            l.get_output_numbers_2()
        ).map(|v| 
            v.iter()
            .enumerate()
            .map(|(idx, &i)| 
                10_i32.pow(3 - idx as u32) * i
            ).sum::<i32>()
        ).sum::<i32>();

    format!("{}", count)
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    let sample = parse_input(read_file("sample"));
    let input = parse_input(read_file("input"));

    assert_eq!(part1(&sample), "26");
    formatted_print("A", part1(&input));

    assert_eq!(part2(&sample), "61229");
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