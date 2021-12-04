use std::fmt;
use std::collections::HashMap;

// ========================= Challenge Logic ============================

// #[derive(Copy, Clone)]
pub struct BingoBoard {
    cells: [bool; 25],
    mapping: HashMap<usize, usize>,
    reverse_mapping: HashMap<usize, usize>,
}

impl BingoBoard {
    pub fn from_str(board: &str) -> BingoBoard {
        let cells = [false; 25];
        let mut mapping = HashMap::new();
        let mut reverse_mapping = HashMap::new();

        for (j, l) in board.lines().enumerate() {
            for (i, c) in l.split_whitespace().enumerate() {
                let k = c.parse::<usize>().expect("Could not parse number of cell");
                mapping.insert(k,  j * 5 + i);
                reverse_mapping.insert(j * 5 + i, k);
            }
        }

        BingoBoard {
            cells,
            mapping,
            reverse_mapping
        }
    }

    pub fn check_rows(&self) -> bool {
        for j in 0..5 {
            let mut bingo = true;

            for i in 0..5 {
                bingo &= self.cells[j * 5 + i];
            }

            if bingo {
                return true;
            }
        }
        
        return false;
    }

    pub fn check_cols(&self) -> bool {
        for i in 0..5 {
            let mut bingo = true;
            
            for j in 0..5 {
                bingo &= self.cells[j * 5 + i];
            }

            if bingo {
                return true;
            }
        }
        
        return false;
    }

    pub fn check_diags(&self) -> bool {
        self.cells[0 * 5 + 0] && self.cells[1 * 5 + 1] && self.cells[2 * 5 + 2] && self.cells[3 * 5 + 3] && self.cells[4 * 5 + 4] ||
        self.cells[0 * 5 + 4] && self.cells[1 * 5 + 3] && self.cells[2 * 5 + 2] && self.cells[3 * 5 + 1] && self.cells[4 * 5 + 0]
    }

    pub fn bingo(&self) -> bool {
        self.check_rows() || self.check_cols() //|| self.check_diags()
    }

    pub fn mark(&mut self, idx: usize) {
        match self.mapping.get(&idx) {
            Some(&cell_idx) => self.cells[cell_idx] = true,
            None => {},
        }
    }
    
    pub fn get_unmarked(&self) -> Vec<usize> {
        let mut result = Vec::new();

        for j in 0..5 {
            for i in 0..5 {
                if !self.cells[j * 5 + i] {
                    result.push(self.reverse_mapping[&(j * 5 + i)]);
                }
            }
        }

        return result;
    }
}

impl fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:>2} {:>2} {:>2} {:>2} {:>2}\n", self.reverse_mapping[&0], self.reverse_mapping[&1], self.reverse_mapping[&2], self.reverse_mapping[&3], self.reverse_mapping[&4])?;
        write!(f, "{:>2} {:>2} {:>2} {:>2} {:>2}\n", self.reverse_mapping[&5], self.reverse_mapping[&6], self.reverse_mapping[&7], self.reverse_mapping[&8], self.reverse_mapping[&9])?;
        write!(f, "{:>2} {:>2} {:>2} {:>2} {:>2}\n", self.reverse_mapping[&10], self.reverse_mapping[&11], self.reverse_mapping[&12], self.reverse_mapping[&13], self.reverse_mapping[&14])?;
        write!(f, "{:>2} {:>2} {:>2} {:>2} {:>2}\n", self.reverse_mapping[&15], self.reverse_mapping[&16], self.reverse_mapping[&17], self.reverse_mapping[&18], self.reverse_mapping[&19])?;
        write!(f, "{:>2} {:>2} {:>2} {:>2} {:>2}\n", self.reverse_mapping[&20], self.reverse_mapping[&21], self.reverse_mapping[&22], self.reverse_mapping[&23], self.reverse_mapping[&24])?;

        Ok(())
    }
}

pub fn part1(input: String) -> String {
    let lines = input.lines().collect::<Vec<&str>>();

    let drawing: Vec<usize> = lines[0].split(",").map(|v|v.parse::<usize>().expect("Could not parse drawing")).collect();

    let mut boards: Vec<BingoBoard> = Vec::new();

    for i in (2..lines.len()).step_by(6) {
        let board_string = String::from("") + lines[i] + "\n" + lines[i+1] + "\n" + lines[i+2] + "\n" + lines[i+3] + "\n" + lines[i+4];
        boards.push(BingoBoard::from_str(&board_string));
    }

    for d in drawing {
        for b in boards.iter_mut() {
            b.mark(d);
            if b.bingo() {
                return format!("{}", b.get_unmarked().into_iter().sum::<usize>() * d)
            }
        }
        
    }
    
    format!("No solution found!")
}

pub fn part2(input: String) -> String {
    let lines = input.lines().collect::<Vec<&str>>();

    let drawing: Vec<usize> = lines[0].split(",").map(|v|v.parse::<usize>().expect("Could not parse drawing")).collect();

    let mut boards: Vec<BingoBoard> = Vec::new();

    for i in (2..lines.len()).step_by(6) {
        let board_string = String::from("") + lines[i] + "\n" + lines[i+1] + "\n" + lines[i+2] + "\n" + lines[i+3] + "\n" + lines[i+4];
        boards.push(BingoBoard::from_str(&board_string));
    }

    for d in drawing {
        boards.iter_mut().for_each(|b| b.mark(d));
        for b in boards.iter() {
            if b.bingo() && boards.len() == 1 {
                return format!("{}", b.get_unmarked().into_iter().sum::<usize>() * d)
            }
        }
        boards = boards.drain(..).filter(|b| !b.bingo()).collect::<Vec<BingoBoard>>();
        
    }
    
    format!("No solution found!")
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