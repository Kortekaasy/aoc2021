use std::collections::HashMap;
use petgraph::{Graph, Undirected, graph::NodeIndex, dot::Dot, dot::Config, visit::{Bfs, EdgeRef}, prelude::EdgeIndex};

// ========================= Challenge Logic ============================

pub fn parse_input(input: String) -> Vec<Vec<u32>> {
    input.lines()
        .map(|s| 
            s.chars()
            .map(|c| 
                c.to_digit(10).unwrap()
            ).collect::<Vec<u32>>()
        ).collect()
}

pub fn part1(input: &Vec<Vec<u32>>) -> String {
    let mut g = Graph::<u32, (), Undirected>::new_undirected();

    let nodes = input.iter()
        .map(|v|
            v.iter().map(|d|
                g.add_node(*d)
            ).collect::<Vec<NodeIndex>>()
        ).collect::<Vec<Vec<NodeIndex>>>();
    
    for j in 0..nodes.len() {
        for i in 0..nodes[j].len() {
            if i > 0 {
                g.add_edge(nodes[j][i-1], nodes[j][i], ());
            }
            if j > 0 {
                g.add_edge(nodes[j-1][i], nodes[j][i], ());
            }
        }
    }

    let mut lowest: Vec<NodeIndex> = Vec::new();

    for j in 0..nodes.len() {
        for i in 0..nodes[j].len() {
            let idx = nodes[j][i];
            if g.neighbors(idx).all(|n|g[idx] < g[n]) {
                lowest.push(idx);
            }
        }
    }

    format!("{}", lowest.iter().map(|&n| g[n] + 1).sum::<u32>())
}

pub fn map_changed(original: &HashMap<usize, HashMap<usize, i32>>, updated: &HashMap<usize, HashMap<usize, i32>>) -> bool {
    let y_max = original.len();
    let x_max = original[&0].len();
    for j in 0..y_max {
        for i in 0..x_max {
            if original[&j][&i] != updated[&j][&i] {
                return true;
            }
        }
    }
    return false;
}

pub fn part2(input: &Vec<Vec<u32>>) -> String {
    let mut g = Graph::<u32, (), Undirected>::new_undirected();

    let nodes = input.iter()
        .map(|v|
            v.iter().map(|d|
                g.add_node(*d)
            ).collect::<Vec<NodeIndex>>()
        ).collect::<Vec<Vec<NodeIndex>>>();
    
    for j in 0..nodes.len() {
        for i in 0..nodes[j].len() {
            if i > 0 {
                g.add_edge(nodes[j][i-1], nodes[j][i], ());
            }
            if j > 0 {
                g.add_edge(nodes[j-1][i], nodes[j][i], ());
            }
        }
    }

    let mut lowest: Vec<NodeIndex> = Vec::new();

    for j in 0..nodes.len() {
        for i in 0..nodes[j].len() {
            let idx = nodes[j][i];
            if g.neighbors(idx).all(|n|g[idx] < g[n]) {
                lowest.push(idx);
            }
        }
    }

    // remove edges with '9' nodes
    let mut to_remove: Vec<EdgeIndex> = Vec::new();
    loop {
        for j in 0..nodes.len() {
            for i in 0..nodes[j].len() {
                let idx = nodes[j][i];
                if g[idx] == 9 {
                    let edges = g.edges(idx);
                    for e in edges {
                        println!("Remove edge {:?} of node {:?}", e.id(), idx);
                        to_remove.push(e.id());
                    }
                }
            }
        }
        
        if to_remove.len() == 0 {
            break;
        }
        to_remove.drain(..).for_each(|e| {g.remove_edge(e);});
    }

    let mut basin_sizes = Vec::new();
    for l in lowest {
        let mut bfs = Bfs::new(&g, l);
        let mut basin_size = 0;
        println!("Neighbours for {:?}:", l);
        while let Some(_nx) = bfs.next(&g) {
            println!("\t{:?}", _nx);
            basin_size += 1;
        }
        basin_sizes.push(basin_size);
    }
    basin_sizes.sort_by_key(|&k| std::cmp::Reverse(k));

    println!("{:?}", basin_sizes);
    
    println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
    format!("{}", basin_sizes[0] * basin_sizes[1] * basin_sizes[2])
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    let sample = parse_input(read_file("sample"));
    let input = parse_input(read_file("input"));
    formatted_print("A", part1(&input));
    formatted_print("B", part2(&sample));
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