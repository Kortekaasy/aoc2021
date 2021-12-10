use std::collections::HashMap;

// ========================= Challenge Logic ============================

pub fn part1(input: String) -> String {
    let mut map: HashMap<usize,HashMap<usize, u32>> = HashMap::new();
    input.lines()
        .enumerate()
        .for_each(|(j, s)| 
            s.chars()
             .enumerate()
             .for_each(|(i, c)| {
                map.entry(j).or_insert(HashMap::new()).entry(i).or_insert_with(|| c.to_digit(10).unwrap());
             })
        );
    
    let y_max = map.len();
    let x_max = map[&0].len();

    let mut sum = 0;

    for j in 0..y_max {
        for i in 0..x_max {
            let mut res = true;
            let val = map.get(&j.clone()).unwrap().get(&i.clone()).unwrap();
            for shift in &[-1, 1] {
                if let Some(m) = map.get(&((j as isize + shift) as usize)) {
                    if let Some(v) = m.get(&i.clone()) {
                        res &= val < v;
                    }
                }
            }

            for shift in &[-1, 1] {
                if let Some(m) = map.get(&j.clone()) {
                    if let Some(v) = m.get(&((i as isize + shift) as usize)) {
                        res &= val < v;
                    }
                }
            }

            if res {
                sum += *val + 1;
            }
        }
    }

    format!("{}", sum)
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

pub fn part2(input: String) -> String {
    let mut map: HashMap<usize,HashMap<usize, u32>> = HashMap::new();
    let mut basins: HashMap<usize,HashMap<usize, i32>> = HashMap::new();
    input.lines()
        .enumerate()
        .for_each(|(j, s)| 
            s.chars()
            .enumerate()
            .for_each(|(i, c)| {
                map.entry(j).or_insert(HashMap::new()).entry(i).or_insert_with(|| c.to_digit(10).unwrap());
                basins.entry(j).or_insert(HashMap::new()).entry(i).or_insert_with(|| -1);
            })
        );
    
    let y_max = map.len();
    let x_max = map[&0].len();

    let mut basin_id_ctr = 0;

    // Find lowest point, and mark their label in the basins map
    for j in 0..y_max {
        for i in 0..x_max {
            let mut res = true;
            let val = map.get(&j.clone()).unwrap().get(&i.clone()).unwrap();
            for shift in &[-1, 1] {
                if let Some(m) = map.get(&((j as isize + shift) as usize)) {
                    if let Some(v) = m.get(&i.clone()) {
                        res &= val < v;
                    }
                }
            }

            for shift in &[-1, 1] {
                if let Some(m) = map.get(&j.clone()) {
                    if let Some(v) = m.get(&((i as isize + shift) as usize)) {
                        res &= val < v;
                    }
                }
            }

            if res {
                *basins.entry(j).or_insert(HashMap::new()).entry(i).or_default() = basin_id_ctr;
                basin_id_ctr += 1;

            }
        }
    }

    let mut original_basins = basins.clone();
    let mut l = true;
    // Keep propagating basin labels until the no more labels are updated
    while l {
        for j in 0..y_max {
            for i in 0..x_max {
                let val = map.get(&j.clone()).unwrap().get(&i.clone()).unwrap();
                // Skip 9's
                if val == &9 {
                    continue;
                }
    
                // Skip already assigned
                if basins[&j][&i] != -1 {
                    continue;
                }
    
                let mut basin = -1;
    
                // Check top & bottom
                for &shift in &[-1, 1] {
                    if let Some(m) = basins.get(&((j as isize + shift) as usize)) {
                        if let Some(v) = m.get(&i) {
                            if *v != -1 && basin == -1{
                                basin = *v;
                            }
                        }
                    }
                }
    
                // Check left & right
                for &shift in &[-1, 1] {
                    if let Some(m) = basins.get(&j) {
                        if let Some(v) = m.get(&((i as isize + shift) as usize)) {
                            if *v != -1 && basin == -1 {
                                basin = *v;
                            }
                        }
                    }
                }
                
                // if no neighbour was found, skip this one for now
                if basin == -1 {
                    continue;
                }
    
                
                *basins.entry(j).or_insert(HashMap::new()).entry(i).or_default() = basin;
            }
        }

        l = map_changed(&original_basins, &basins);
        original_basins = basins.clone();
        
    }
    
    let mut basin_ids: HashMap<i32, i32> = HashMap::new();
    for j in 0..y_max {
        for i in 0..x_max {
            let b_id = basins[&j][&i];
            *basin_ids.entry(b_id).or_insert(0) += 1;
        }
    }

    let mut basin_sizes = basin_ids.iter().filter(|&(k, _v)| *k != -1).map(|(_k, v)| *v).collect::<Vec<i32>>();
    basin_sizes.sort_by_key(|&k| std::cmp::Reverse(k));

    format!("{}", basin_sizes[0] * basin_sizes[1] * basin_sizes[2])
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