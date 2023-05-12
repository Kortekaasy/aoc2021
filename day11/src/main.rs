use std::collections::HashSet;
// ========================= Challenge Logic ============================
static WIDTH: usize = 10;

#[derive(Clone)]
pub struct Octopus {
    light: u32,
    neighbours: Vec<usize>
}

impl Octopus {

    fn new(light: u32, neighbours: Vec<usize>) -> Octopus {
        Octopus {
            light,
            neighbours
        }
    }

    fn get_energy(&self) -> u32 {
        self.light
    }

    fn reset_energy(&mut self) {
        self.light = 0;
    }

    fn increase_energy(&mut self) {
        self.light += 1;
    }

    fn flash(&self) -> bool {
        self.light == 10
    }

    fn get_neighbours(&self) -> Vec<usize> {
        self.neighbours.clone()
    }
    
}

pub fn to_index(i: i32, j: i32) -> std::option::Option<usize> {
    if (0..WIDTH as i32).contains(&i) && (0..WIDTH as i32).contains(&j) {
        Some(j as usize * WIDTH + i as usize)
    } else {
        None
    }
}

pub fn parse_input(input: String) -> Vec<Octopus> {
    let mut octopi = Vec::new();

    for (j, l) in input.lines().enumerate() {
        for (i, c) in l.chars().enumerate() {
            let light = c.to_digit(10).expect("Expected to parse char as digit");
            let mut neighbours = Vec::new();
            for k in [-1, 0, 1] {
                for l in [-1, 0, 1] {
                    if !(l == 0 && k == 0) {
                        if let Some(idx) = to_index(i as i32 + l, j as i32 + k) {
                            neighbours.push(idx);
                        }
                    }
                }
            }
            octopi.push(Octopus::new(light, neighbours));
        }
    }

    octopi
}

pub fn print_octopi(octopi: &Vec<Octopus>, round: usize) {
    if round == 0 {
        println!("Before any steps");
        for j in 0..WIDTH {
            for i in 0..WIDTH {
                print!("{}", octopi[j * WIDTH + i].get_energy());
            }
            println!("");
        }
        println!("");
    } else {
        println!("After Round {}", round);
        for j in 0..WIDTH {
            for i in 0..WIDTH {
                print!("{}", octopi[j * WIDTH + i].get_energy());
            }
            println!("");
        }
        println!("");
    }

}

pub fn part1(mut octopi: Vec<Octopus>) -> String {
    let mut flashes = 0;

    // print_octopi(&octopi, 0);

    for round in 0..100 {
        octopi.iter_mut()
            .for_each(|o| o.increase_energy());

        let mut flashing = octopi.iter().enumerate().filter(|&(_i, o)| o.flash()).map(|(i, _o)| i).collect::<HashSet<usize>>();
        flashes += flashing.len();

        while !flashing.is_empty() {
            let mut new_flashing = HashSet::new();
            for i in flashing.drain(){
                for n in octopi[i].get_neighbours() {
                    octopi[n].increase_energy();
                    if octopi[n].flash() {
                        flashes += 1;
                        new_flashing.insert(n);
                    }
                }
            }
            flashing = new_flashing;
            // flashing = octopi.iter().enumerate().filter(|&(_i, o)| o.flash()).map(|(i, _o)| i).collect::<Vec<usize>>();
        }

        octopi.iter_mut()
            .filter(|o| o.get_energy() > 9)
            .for_each(|o|o.reset_energy());

        // print_octopi(&octopi, round + 1);
    }

    format!("{}", flashes)
}

pub fn part2(mut octopi: Vec<Octopus>) -> String {
    let mut flashes = 0;
    let mut prev_flashes = 0;

    let mut round = 0;

    while flashes - prev_flashes != WIDTH * WIDTH {
        round += 1;
        prev_flashes = flashes;

        octopi.iter_mut()
            .for_each(|o| o.increase_energy());

        let mut flashing = octopi.iter().enumerate().filter(|&(_i, o)| o.flash()).map(|(i, _o)| i).collect::<HashSet<usize>>();
        flashes += flashing.len();

        while !flashing.is_empty() {
            let mut new_flashing = HashSet::new();
            for i in flashing.drain(){
                for n in octopi[i].get_neighbours() {
                    octopi[n].increase_energy();
                    if octopi[n].flash() {
                        flashes += 1;
                        new_flashing.insert(n);
                    }
                }
            }
            flashing = new_flashing;
        }

        octopi.iter_mut()
            .filter(|o| o.get_energy() > 9)
            .for_each(|o|o.reset_energy());
    }

    format!("{}", round)
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    let sample = parse_input(read_file("sample"));
    let input = parse_input(read_file("input"));

    assert_eq!(part1(sample.clone()), "1656");
    formatted_print("A", part1(input.clone()));


    assert_eq!(part2(sample.clone()), "195");
    formatted_print("B", part2(input.clone()));
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