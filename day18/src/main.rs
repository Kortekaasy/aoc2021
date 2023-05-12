use std::ops::Add;
use std::fmt;
use pest_consume::{Parser, Error, match_nodes};
// ========================= Challenge Logic ============================
#[derive(Parser)]
#[grammar = "../grammar.pest"]
struct SnailFishNumberParser;

type ParseResult<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;
#[pest_consume::parser]
impl SnailFishNumberParser {
    fn EOI(_input: Node) -> ParseResult<()> {
        Ok(())
    }

    fn num(input: Node) -> ParseResult<i64> {
        input.as_str()
            .parse::<i64>()
            .map_err(|e| input.error(e))
    }

    fn sfnum(input: Node) -> ParseResult<SnailFishNumber> {
        Ok(match_nodes!(input.into_children();
            [num(l), num(r)] => SnailFishNumber::from(l, r),
            [sfnum(l), num(r)] => l + r,
            [num(l), sfnum(r)] => l + r,
            [sfnum(l), sfnum(r)] => l + r,
        ))
    }

    fn sfnums(input: Node) -> ParseResult<Vec<SnailFishNumber>> {
        Ok(match_nodes!(input.into_children();
            [sfnum(sfnums).., EOI(_)] => sfnums.collect(),
        ))
    }
}

#[derive(Clone, Debug)]
pub enum TreeValue {
    Leaf(i64),
    Branch(TreeNode)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Explosion {
    No,
    Yes,
    Child
}

#[derive(Clone)]
pub struct TreeNode {
    left: Box<TreeValue>,
    right: Box<TreeValue>,
}

impl TreeNode {
    fn from(left: i64, right: i64) -> TreeNode {
        TreeNode { 
            left: Box::new(TreeValue::Leaf(left)), 
            right: Box::new(TreeValue::Leaf(right)) 
        }
    }

    fn magnitude(&self) -> i64 {
        match (self.left.as_ref(), self.right.as_ref()) {
            (TreeValue::Leaf(l), TreeValue::Leaf(r)) => 3 * l + 2 * r,
            (TreeValue::Leaf(l), TreeValue::Branch(r)) => 3 * l + 2 * r.magnitude(),
            (TreeValue::Branch(l), TreeValue::Leaf(r)) => 3 * l.magnitude() + 2 * r,
            (TreeValue::Branch(l), TreeValue::Branch(r)) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }

    fn reduce(mut self) -> TreeNode {
        // println!("\nAfter Addition: {}", self);
        loop {
            // If we have nothing to explode
            let (_, explosion) = self.explode(0);
            if explosion == Explosion::No {
                // Check if we have something to split
                if !self.split() {
                    // println!("After Reduce:   {}", self);
                    break;
                } else {
                    // println!("After split:    {}", self);
                }
            } else {
                // println!("After explode:  {}", self);
            }
        }
        self
    }

    fn explode(&mut self, level: usize) -> ((i64, i64), Explosion) {
        // Just propagate to children if needed
        if level >= 4 {
            let explode = match (self.left.as_ref(), self.right.as_ref()) {
                (TreeValue::Leaf(_l), TreeValue::Leaf(_r)) => true,
                (_, _) => false
            };
            
            if explode {
            // We need to explode!
                let l = match self.left.as_mut() {
                    TreeValue::Leaf(i) => *i,
                    TreeValue::Branch(_b) => panic!("Should not be possible")
                };

                let r = match self.right.as_mut() {
                    TreeValue::Leaf(i) => *i,
                    TreeValue::Branch(_b) => panic!("Should not be possible")
                };
                return ((l, r), Explosion::Child);
            }
            
        }

        // Check first on the left for explosions
        let ((ll, lr), explosion) = match self.left.as_mut() {
            TreeValue::Leaf(_) => ((0, 0), Explosion::No),
            TreeValue::Branch(b) => b.explode(level + 1)
        };

        // if something exploded on the left branch
        if explosion != Explosion::No {
            // if child exploded, replace with zero
            if explosion == Explosion::Child {
                self.left = Box::new(TreeValue::Leaf(0));
            }
            
            match self.right.as_mut() {
                TreeValue::Leaf(i) => *i += lr,
                TreeValue::Branch(b) => b.add_to_first_right(lr)
            };
            
            return ((ll, 0), Explosion::Yes);
        }

        // Then check on the right for explosions
        let ((rl, rr), explosion) = match self.right.as_mut() {
            TreeValue::Leaf(_) => ((0, 0), Explosion::No),
            TreeValue::Branch(b) => b.explode(level + 1)
        };

        // if something exploded on the left branch
        if explosion != Explosion::No {
            // if child exploded, replace with zero
            if explosion == Explosion::Child {
                self.right = Box::new(TreeValue::Leaf(0));
            }

            match self.left.as_mut() {
                TreeValue::Leaf(i) => *i += rl,
                TreeValue::Branch(b) => b.add_to_first_left(rl)
            };
            return ((0, rr), Explosion::Yes);
        }

        return ((0, 0), Explosion::No);
        
    }

    fn add_to_first_left(&mut self, val: i64) {
        match self.right.as_mut() {
            TreeValue::Leaf(i) => *i += val,
            TreeValue::Branch(b) => b.add_to_first_left(val)
        }
    }

    fn add_to_first_right(&mut self, val: i64) {
        match self.left.as_mut() {
            TreeValue::Leaf(i) => *i += val,
            TreeValue::Branch(b) => b.add_to_first_right(val)
        }
    }

    fn split(&mut self) -> bool {
        if !match self.left.as_mut() {
            TreeValue::Leaf(i) if *i > 9 => {
                self.left = Box::new(
                    TreeValue::Branch(TreeNode {
                        left: Box::new(TreeValue::Leaf(*i / 2)),
                        right: Box::new(TreeValue::Leaf((*i + 1) / 2)),
                }));
                true
            },
            TreeValue::Leaf(i) => false,
            TreeValue::Branch(b) => b.split()
        } {
            match self.right.as_mut() {
                TreeValue::Leaf(i) if *i > 9 => {
                    self.right = Box::new(
                    TreeValue::Branch(TreeNode {
                        left: Box::new(TreeValue::Leaf(*i / 2)),
                        right: Box::new(TreeValue::Leaf((*i + 1) / 2)),
                    }));
                    true
                },
                TreeValue::Leaf(i) => false,
                TreeValue::Branch(b) => b.split()
            }
        } else {
            true
        }
    }
}

type SnailFishNumber = TreeNode;

impl Add<TreeNode> for TreeNode {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            left: Box::new(TreeValue::Branch(self)),
            right: Box::new(TreeValue::Branch(other))
        }.reduce()
    }
}

impl Add<i64> for TreeNode {
    type Output = Self;

    fn add(self, other: i64) -> Self::Output {
        Self::Output {
            left: Box::new(TreeValue::Branch(self)),
            right: Box::new(TreeValue::Leaf(other))
        }.reduce()
    }
}

impl Add<TreeNode> for i64 {
    type Output = TreeNode;

    fn add(self, other: TreeNode) -> Self::Output {
        Self::Output {
            left: Box::new(TreeValue::Leaf(self)),
            right: Box::new(TreeValue::Branch(other)),
        }.reduce()
    }
}

impl fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}, {:?}]", self.left, self.right)?;
        Ok(())
    }
}

impl fmt::Display for TreeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TreeValue::Branch(b) =>  write!(f, "[{}, {}]", b.left, b.right)?,
            TreeValue::Leaf(i) => write!(f, "{}", i)?
        }
       
        Ok(())
    }
}

impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.left, self.right)?;
        Ok(())
    }
}

pub fn parse_input(input: String) -> ParseResult<Vec<SnailFishNumber>> {
    let inputs = SnailFishNumberParser::parse(Rule::sfnums, &input)?;
    let input = inputs.single()?;
    SnailFishNumberParser::sfnums(input)
}

pub fn part1(input: &Vec<SnailFishNumber>) -> String {
    let mut cloned = input.clone();
    let init = cloned.remove(0);
    let sum = cloned.drain(..).fold(init, |acc, elem| acc + elem);
    
    format!("{}", sum.magnitude())
}

pub fn part2(input: &Vec<SnailFishNumber>) -> String {
    let mut max = i64::MIN;
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i != j {
                max = max.max(
                    (input[i].clone() + input[j].clone()).magnitude()
                )
            }
        }
    }
    // let max = (0..input.len())
    //     .map(|j|
    //         (0..input.len())
    //         .filter(|i| i != &j)
    //         .map(|i|
    //             (input[i].clone() + input[j].clone()).magnitude()
    //         ).max().expect("Expected to find max")
    //     ).max().expect("Expected to find max");
    format!("{}", max)
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() -> ParseResult<()> {
    let sample = parse_input(read_file("sample"))?;
    let input = parse_input(read_file("input"))?;

    assert_eq!(part1(&sample), "4140");
    formatted_print("A", part1(&input));

    assert_eq!(part2(&sample), "3993");
    formatted_print("B", part2(&input));

    Ok(())
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