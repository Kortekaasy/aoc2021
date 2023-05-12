// ========================= Challenge Logic ============================

pub fn parse_input(input: String) -> (usize, usize) {
    let mut lines = input.lines();
    let pos1 = lines
        .next()
        .expect("Expected line 1")
        .split(" ")
        .last()
        .expect("Expected position")
        .parse::<usize>()
        .expect("Expected to parse pos 1");


    let pos2 = lines
        .next()
        .expect("Expected line 2")
        .split(" ")
        .last()
        .expect("Expected position")
        .parse::<usize>()
        .expect("Expected to parse pos 2");
    
    (pos1, pos2)
}

pub struct Die {
    counter: usize,
    rolls: usize,
}

impl Die {
    fn new() -> Die {
        Die { counter: 0, rolls: 0 }
    }

    fn roll(&mut self) -> usize {
        self.rolls += 1;
        self.counter += 1;
        let res = self.counter;
        self.counter %= 100;
        res
    }

    fn roll3(&mut self) -> [usize; 3] {
        [self.roll(), self.roll(), self.roll()]
    }

    fn get_rolls(&self) -> usize {
        self.rolls
    }
}

pub fn mod_pos(x: usize) -> usize {
    (x.wrapping_sub(1) % 10).wrapping_add(1)
}

pub fn part1(input: &(usize, usize)) -> String {
    let mut pos = [input.0, input.1];
    let mut die = Die::new();
    let mut score= [0, 0];

    let mut j = 1;

    let mut finished = false;

    while !finished {
        for i in 0..2 {
            for r in die.roll3() {
                pos[i] += r;
            }
            pos[i] = mod_pos(pos[i]);
            score[i] += pos[i];
            if score[i] >= 1000 {
                finished = true;
                break;
            }
        }
        j += 1;

    }
    
    format!("{}", score[0].min(score[1]) * die.get_rolls())
}

pub fn part2(input: &(usize, usize)) -> String {
    let mut pos = [input.0, input.1];
    println!("pos: {:?}", pos);
    let mut positions: [[usize; 32]; 2] =  [[0; 32]; 2];

    for j in 0..2 {
        // Roll 1
        positions[j][mod_pos(pos[j] + 1)] += 1;
        positions[j][mod_pos(pos[j] + 2)] += 1;
        positions[j][mod_pos(pos[j] + 3)] += 1;

        println!("{:?}", positions);

        // Roll 2 & 3
        for _ in [1, 2] {
            let old_positions = positions;
            for i in 0..29 {
                positions[j][i + 1] += old_positions[j][i];
                positions[j][i + 2] += old_positions[j][i];
                positions[j][i + 3] += old_positions[j][i];
            }
        }
    }
    println!("{:?}", positions);

    for _ in 0..20 {
        for j in 0..2 {
            for _ in [0, 1, 2] {
                let old_positions = positions;
                for i in 0..29 {
                    positions[j][i + 1] += old_positions[j][i];
                    positions[j][i + 2] += old_positions[j][i];
                    positions[j][i + 3] += old_positions[j][i];
                }
            }
        }
        println!("{:?}", positions);
    }
    
    // println!("{:?}", scores);
    format!("Unimplemented!")
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    let sample = parse_input(read_file("sample"));
    let input = parse_input(read_file("input"));

    assert_eq!(part1(&sample), "739785");
    formatted_print("A", part1(&input));

    assert_eq!(part2(&sample), "444356092776315");
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