use std::collections::HashMap;
// ========================= Challenge Logic ============================

type Rules = HashMap<(char, char), ((char, char), (char, char))>;

pub fn parse_input(input: String) -> (String, Rules) {
    let mut lines = input.lines();
    let template = String::from(lines.next().expect("Expected Template"));

    // skip newline
    lines.next();

    let mut mapping: Rules = HashMap::new();
    for l in lines {
        let (lhs, rhs) = l.split_once(" -> ").expect("Expected relation");
        let lhs_chars = lhs.chars().collect::<Vec<char>>();
        let middle = rhs.chars().next().unwrap();
        mapping.insert((lhs_chars[0], lhs_chars[1]), ((lhs_chars[0], middle), (middle, lhs_chars[1])));
    }

    (template, mapping)
}

pub fn part1(template: &String, rules: &Rules) -> String {
    // Create counts map
    let mut counts: HashMap<(char, char), u64> = rules.keys().map(|&k| (k, 0)).collect();

    // first add template to count
    template
        .chars()
        .zip(
            template.chars().skip(1)
        ).for_each(|k|
            *counts.entry(k).or_default() += 1
        );
    
    // Perform 40 rounds of polymer improvement
    for _r in 0..10 {
        let mut new_counts = HashMap::new();
        for (k, v) in counts.drain() {
            let (new_k1, new_k2) = rules[&k];
            *new_counts.entry(new_k1).or_default() += v;
            *new_counts.entry(new_k2).or_default() += v;
        }
        counts = new_counts;
    }

    // Compute frequency of all char pairs
    let mut freqs = HashMap::<char, u64>::new();
    counts.iter().for_each(|(&k, &v)| {
        // Only take the last char of each pair, otherwise we count chars double
        *freqs.entry(k.1).or_default() += v;
    });
    // First char won't be counted with above approach, to account for that
    *freqs.entry(template.chars().next().unwrap()).or_default() += 1;

    // Compute min and max char, and return result
    let min_char = *freqs.values().min().expect("Expected min freq");
    let max_char = *freqs.values().max().expect("Expected min freq");
    format!("{}", max_char - min_char)
}

pub fn part2(template: &String, rules: &Rules) -> String {
    // Create counts map
    let mut counts: HashMap<(char, char), u64> = rules.keys().map(|&k| (k, 0)).collect();

    // first add template to count
    template
        .chars()
        .zip(
            template.chars().skip(1)
        ).for_each(|k|
            *counts.entry(k).or_default() += 1
        );
    
    // Perform 40 rounds of polymer improvement
    for _r in 0..40 {
        let mut new_counts = HashMap::new();
        for (k, v) in counts.drain() {
            let (new_k1, new_k2) = rules[&k];
            *new_counts.entry(new_k1).or_default() += v;
            *new_counts.entry(new_k2).or_default() += v;
        }
        counts = new_counts;
    }

    // Compute frequency of all char pairs
    let mut freqs = HashMap::<char, u64>::new();
    counts.iter().for_each(|(&k, &v)| {
        // Only take the last char of each pair, otherwise we count chars double
        *freqs.entry(k.1).or_default() += v;
    });
    // First char won't be counted with above approach, to account for that
    *freqs.entry(template.chars().next().unwrap()).or_default() += 1;

    // Compute min and max char, and return result
    let min_char = *freqs.values().min().expect("Expected min freq");
    let max_char = *freqs.values().max().expect("Expected min freq");
    format!("{}", max_char - min_char)
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    let (s_template, s_rules) = parse_input(read_file("sample"));
    let (i_template, i_rules) = parse_input(read_file("input"));

    assert_eq!(part1(&s_template, &s_rules), "1588");
    formatted_print("A", part1(&i_template, &i_rules));

    assert_eq!(part2(&s_template, &s_rules), "2188189693529");
    formatted_print("B", part2(&i_template, &i_rules));
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