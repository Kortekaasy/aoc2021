// ========================= Challenge Logic ============================

pub fn parse_input(input: String) -> Vec<((usize, usize), (usize, usize))> {
    input.lines().map(|l| {
        let splitted = l.splitn(3, " ").collect::<Vec<&str>>();
        let (start, _arrow, end) = (splitted[0], splitted[1], splitted[2]);
        let (start_x, start_y) = start.split_once(",").map(|(x, y)| (x.parse::<usize>().expect("could not parse start x"), y.parse::<usize>(). expect("could not parse start y"))).expect("Could not parse start coordinates");
        let (end_x, end_y) = end.split_once(",").map(|(x, y)| (x.parse::<usize>().expect("could not parse end x"), y.parse::<usize>(). expect("could not parse end y"))).expect("Could not parse end coordinates");
        ((start_x, start_y), (end_x, end_y))
    }).collect::<Vec<((usize, usize), (usize, usize))>>()
}

pub fn part1(coords: &Vec<((usize, usize), (usize, usize))>) -> String {

    let mut field: [[u32; 1000]; 1000] = [[0; 1000]; 1000];

    for &((x1, y1), (x2, y2)) in coords {
        if x1 != x2 && y1 != y2 {
            continue;
        }

        let dx = if x1 < x2 { 1 } else if x1 == x2 { 0 } else  { -1 };
        let dy = if y1 < y2 { 1 } else if y1 == y2 { 0 } else  { -1 };

        let (mut x, mut y) = (x1, y1);
        loop {
            field[y][x] += 1;
            x = (x as i64 + dx) as usize;
            y = (y as i64 + dy) as usize;

            if x == x2 && y == y2 {
                field[y][x] += 1;
                break;
            }
        }
    }
    
    format!("{}", field.iter().map(|r| r.iter().filter(|&&c| c > 1).count()).sum::<usize>())
}

pub fn part2(coords: &Vec<((usize, usize), (usize, usize))>) -> String {
    
    let mut field: [[u32; 1000]; 1000] = [[0; 1000]; 1000];

    for &((x1, y1), (x2, y2)) in coords {

        let dx = if x1 < x2 { 1 } else if x1 == x2 { 0 } else { -1 };
        let dy = if y1 < y2 { 1 } else if y1 == y2 { 0 } else { -1 };

        let (mut x, mut y) = (x1, y1);
        loop {
            field[y][x] += 1;
            x = (x as i64 + dx) as usize;
            y = (y as i64 + dy) as usize;

            if x == x2 && y == y2 {
                field[y][x] += 1;
                break;
            }
        }
    }
    
    format!("{}", field.iter().map(|r| r.iter().filter(|&&c| c > 1).count()).sum::<usize>())
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    formatted_print("A", part1(&parse_input(read_file("input"))));
    formatted_print("B", part2(&parse_input(read_file("input"))));
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