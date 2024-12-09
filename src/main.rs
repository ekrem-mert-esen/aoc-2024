use regex::Regex;
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::stdin;
use std::io::Read;
fn read_file(path: &str) -> String {
    let mut file = File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    return contents;
}

fn _d1p1(file: String) -> i32 {
    let lines: Vec<&str> = file.lines().collect();
    let mut left: Vec<i32> = Vec::with_capacity(1000);
    let mut right: Vec<i32> = Vec::with_capacity(1000);
    for line in lines {
        left.push(line.split_whitespace().nth(0).unwrap().parse().unwrap());
        right.push(line.split_whitespace().nth(1).unwrap().parse().unwrap());
    }
    left.sort_unstable();
    right.sort_unstable();
    let sum: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(x, y)| (x - y).abs())
        .sum();
    return sum;
}

fn _d1p2(file: String) -> i64 {
    let lines: Vec<&str> = file.lines().collect();
    let mut left: Vec<i32> = Vec::with_capacity(1000);
    let mut right: Vec<i32> = Vec::with_capacity(1000);
    for line in lines {
        left.push(line.split_whitespace().nth(0).unwrap().parse().unwrap());
        right.push(line.split_whitespace().nth(1).unwrap().parse().unwrap());
    }
    left.sort_unstable();
    right.sort_unstable();
    let mut sum: i64 = 0;
    for x in left.into_iter() {
        let count: i32 = right
            .iter()
            .filter(|&y| *y == x)
            .count()
            .try_into()
            .unwrap();
        sum = sum + (x * count) as i64;
    }
    return sum;
}
fn _d2p1(file: String) -> i32 {
    let lines: Vec<&str> = file.lines().collect();
    let mut sum: i32 = 0;
    for line in lines {
        let report: Vec<i8> = line
            .split_whitespace()
            .filter_map(|i| i.parse::<i8>().ok())
            .collect();
        let increasing = |x: &[i8]| 0 < (x[0] - x[1]) && (x[0] - x[1]) < 4;
        let decreasing = |x: &[i8]| 0 < (x[1] - x[0]) && (x[1] - x[0]) < 4;
        if report.windows(2).all(increasing) || report.windows(2).all(decreasing) {
            sum = sum + 1;
        }
    }
    return sum;
}

fn _d2p2(file: String) -> i32 {
    let lines: Vec<&str> = file.lines().collect();
    let mut sum: i32 = 0;
    for line in lines {
        let report: Vec<i8> = line
            .split_whitespace()
            .filter_map(|i| i.parse::<i8>().ok())
            .collect();
        let increasing = |x: &[i8]| 0 < (x[0] - x[1]) && (x[0] - x[1]) < 4;
        let decreasing = |x: &[i8]| 0 < (x[1] - x[0]) && (x[1] - x[0]) < 4;
        if report.windows(2).all(increasing) || report.windows(2).all(decreasing) {
            sum = sum + 1;
        } else {
            for i in 0..report.len() {
                let mut clone = report.clone();
                clone.remove(i);
                if clone.windows(2).all(increasing) || clone.windows(2).all(decreasing) {
                    sum = sum + 1;
                    break;
                }
            }
        }
    }
    return sum;
}

fn _d3p1(file: String) -> i64 {
    let rexp = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum: i64 = 0;
    for (_, [f1, f2]) in rexp.captures_iter(&file).map(|cap| cap.extract()) {
        sum = sum + (f1.parse::<i64>().unwrap() * f2.parse::<i64>().unwrap());
    }
    return sum;
}

fn _d3p2(file: String) -> i64 {
    let rexp = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let removexp = Regex::new(r"(?s)don't\(\).*?(do\(\)|\z)").unwrap();
    let mut sum: i64 = 0;
    let f = removexp.replace_all(&file, "");
    // let testf = read_file("d3test.txt");
    // assert_eq!(f, testf);
    for (_, [f1, f2]) in rexp.captures_iter(&f).map(|cap| cap.extract()) {
        sum = sum + (f1.parse::<i64>().unwrap() * f2.parse::<i64>().unwrap());
    }
    return sum;
}

fn _d4p1(file: String) -> i32 {
    let charv: Vec<char> = file.chars().collect();
    let range: [i32; 8] = [-143, -142, -141, -1, 1, 141, 142, 143];
    let mut iter: i32 = 0;
    let mut sum = 0;
    for char in charv.clone().into_iter() {
        if char == 'X' {
            for i in range {
                if iter + i < 0 || iter + i > 19877 || iter + (3 * i) < 0 || iter + (3 * i) > 19877
                {
                    continue;
                } else {
                    let nextchar: char = charv[usize::try_from(iter + i).unwrap()];
                    if nextchar == 'M' {
                        let nnchar = charv[usize::try_from(iter + i + i).unwrap()];
                        if nnchar == 'A' {
                            let nnnchar = charv[usize::try_from(iter + i + i + i).unwrap()];
                            if nnnchar == 'S' {
                                sum += 1;
                            }
                        }
                    }
                }
            }
        }
        iter += 1;
    }

    return sum;
}

fn _d4p2(file: String) -> i32 {
    let charv: Vec<char> = file.chars().collect();
    let linelen: i32 = file.lines().nth(0).unwrap().len() as i32;
    let range: [i32; 4] = [-linelen - 3, -linelen - 1, linelen + 1, linelen + 3];
    let mut iter: i32 = 0;
    let mut sum = 0;
    'outer: for char in charv.clone().into_iter() {
        if char == 'A' {
            for i in range {
                if iter + i < 0 || iter + i > charv.len() as i32 {
                    iter += 1;
                    continue 'outer;
                }
            }

            let nwchar: char = charv[usize::try_from(iter + range[0]).unwrap()];
            if nwchar == 'M' {
                let sechar = charv[usize::try_from(iter + range[3]).unwrap()];
                if sechar == 'S' {
                    let swchar = charv[usize::try_from(iter + range[2]).unwrap()];
                    let nechar = charv[usize::try_from(iter + range[1]).unwrap()];
                    if (nechar == 'M' && swchar == 'S') || (nechar == 'S' && swchar == 'M') {
                        sum += 1;
                        iter += 1;
                        continue 'outer;
                    }
                }
            } else if nwchar == 'S' {
                let sechar = charv[usize::try_from(iter + range[3]).unwrap()];
                if sechar == 'M' {
                    let swchar = charv[usize::try_from(iter + range[2]).unwrap()];
                    let nechar = charv[usize::try_from(iter + range[1]).unwrap()];
                    if (nechar == 'M' && swchar == 'S') || (nechar == 'S' && swchar == 'M') {
                        sum += 1;
                        iter += 1;
                        continue 'outer;
                    }
                }
            }
        }

        iter += 1;
    }
    return sum;
}

fn _d5p1(file: String) -> usize {
    let mut lines: Vec<&str> = file.lines().collect();
    lines.sort_unstable();
    let mut ordermap = HashMap::<usize, HashSet<usize>>::new();
    let mut lists: Vec<Vec<usize>> = Vec::with_capacity(200);
    let mut sum: usize = 0;
    for line in lines {
        if line.contains("|") {
            let (l, r) = line.split_once("|").unwrap();
            ordermap
                .entry(r.parse::<usize>().unwrap())
                .or_default()
                .insert(l.parse().unwrap());
        } else if line.contains(",") {
            lists.push(
                line.split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect(),
            );
        }
    }
    for l in lists {
        if l.is_sorted_by(|a, b| ordermap[b].contains(a)) {
            sum = sum + l[l.len() / 2]
        }
    }
    return sum;
}

fn _d5p2(file: String) -> usize {
    let mut lines: Vec<&str> = file.lines().collect();
    lines.sort_unstable();
    let mut ordermap = HashMap::<usize, HashSet<usize>>::new();
    let mut lists: Vec<Vec<usize>> = Vec::with_capacity(200);
    let mut sum: usize = 0;
    for line in lines {
        if line.contains("|") {
            let (l, r) = line.split_once("|").unwrap();
            ordermap
                .entry(r.parse::<usize>().unwrap())
                .or_default()
                .insert(l.parse().unwrap());
        } else if line.contains(",") {
            lists.push(
                line.split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect(),
            );
        }
    }
    for mut l in lists {
        if !(l.is_sorted_by(|a, b| ordermap[b].contains(a))) {
            l.sort_by(|a, b| ordermap[b].contains(a).cmp(&true));
            sum = sum + l[l.len() / 2]
        }
    }
    return sum;
}

fn _d6p1(file: String) -> i32 {
    let mut charv: Vec<char> = file.chars().collect();
    let linelen: i32 = file.lines().nth(0).unwrap().len() as i32;
    let directions: [i32; 4] = [-linelen - 2, 1, linelen + 2, -1];
    let mut pos: i32;
    let mut sum: i32 = 0;
    let mut direction = 0; // # 0=up,1=left,2=right,3=down #

    pos = charv.iter().position(|c| *c == '^').unwrap() as i32;
    charv[pos as usize] = 'X';
    println!("Started at {}", pos);
    while (pos + directions[direction] >= 0)
        && (pos + directions[direction] <= charv.len() as i32)
        && (charv[(pos + directions[direction]) as usize] != '\n')
    {
        if ['X', '.'].contains(&charv[(pos + directions[direction]) as usize]) {
            println!(
                "At {}, trying {}",
                pos,
                charv[(pos + directions[direction]) as usize]
            );
            charv[(pos + directions[direction]) as usize] = 'X';
            pos += directions[direction];
        } else if charv[(pos + directions[direction]) as usize] == '#' {
            direction = (direction + 1) % 4;
            println!("Direction change! Now heading {}", direction);
        }
    }
    let endstate: String = charv.iter().collect();
    println!("End State:");
    println!("{}", endstate);
    for c in charv {
        if c == 'X' {
            sum += 1;
        }
    }
    return sum;
}

fn _d6p2(file: String) -> i32 {
    let mut charv: Vec<char> = file.chars().collect();
    let linelen: i32 = file.lines().nth(0).unwrap().len() as i32;
    let directions: [i32; 4] = [-linelen - 2, 1, linelen + 2, -1];
    let initpos = charv.iter().position(|c| *c == '^').unwrap() as i32;
    let mut pos = initpos;
    let mut sum: i32 = 0;
    let mut direction = 0; // # 0=up,1=right,2=down,3=left #
    let mut simdata: Vec<Vec<char>> = Vec::with_capacity(5500);
    let mut run = 0;

    for i in 0..charv.len() {
        if charv[i] == 'X' {
            charv[i] = '#';
            let mut temp: String = charv.clone().iter().collect();
            // println!(
            //     "Char array: {:?} \
            //           String:{}",
            //     charv, temp
            // );
            // stdin().read(&mut [0]).unwrap();
            temp = temp.replace("X", ".");
            temp = temp.replace(".", "0");
            simdata.push(temp.chars().collect());
            charv[i] = '.';
        }
    }

    'outer: for mut sim in simdata {
        // println!("{}", sim.iter().collect::<String>());
        println!("Starting run #{} ...", run);
        // stdin().read(&mut [0]).unwrap();
        sim[initpos as usize] = '1';
        while (pos + directions[direction] >= 0)
            && (pos + directions[direction] <= sim.len() as i32)
            && (sim[(pos + directions[direction]) as usize] != '\n')
            && (sim[(pos + directions[direction]) as usize] != '\r')
        //took me a day to realise that rust added '\r' to string while converting from char array and back.
        {
            // println!(
            //     "At {}, trying {} at {}",
            //     pos,
            //     sim[(pos + directions[direction]) as usize],
            //     (pos + directions[direction]) as usize,
            // );
            if ['0', '1', '2', '3', '4'].contains(&sim[(pos + directions[direction]) as usize]) {
                let mut d = sim[(pos + directions[direction]) as usize]
                    .to_digit(10)
                    .unwrap();
                d += 1;
                sim[(pos + directions[direction]) as usize] = d.to_string().chars().nth(0).unwrap();
                //              println!("changed to {}", d.to_string().chars().nth(0).unwrap());
                pos += directions[direction];
            } else if sim[(pos + directions[direction]) as usize] == '#' {
                direction = (direction + 1) % 4;
                //  println!("Direction change! Now heading {}", direction);
            } else if sim[(pos + directions[direction]) as usize] == '5' {
                println!("Loop detected! Starting next run");
                sum += 1;
                run += 1;
                pos = initpos;
                direction = 0;
                continue 'outer;
            } else {
                println!("out of options but not out of the loop?");
                println!("State:");
                println!("Direction is {}", direction);
                sim[pos as usize] = '@';
                println!("{}", sim.iter().collect::<String>().replace("0", "."));
                panic!();
            }
        }

        // println!(
        //     "No loop at run #{}, exited from pos {}, direction {}",
        //     run, pos, direction
        // );
        // println!(
        //     "State: \
        //         {}",
        //     sim.iter().collect::<String>().replace("0", ".")
        // );
        //stdin().read(&mut [0]).unwrap();
        run += 1;
        pos = initpos;
        direction = 0;
    }
    return sum;
}

fn _d7recurse(result: u64, operand: u64, operands: &[u64], day2: bool) -> bool {
    //reducing target value by recursively subtracting/dividing by last value & checking if the
    //leftover equals remaining operand at the end.
    if let Some((i, next)) = operands.split_last() {
        [
            if result > *i {
                println!("{} - {}", result, *i);
                Some(result - *i)
            } else {
                None
            },
            if result % *i == 0 {
                println!("{} / {}", result, *i);
                Some(result / *i)
            } else {
                None
            },
            if day2 {
                let j = 10u64.pow(i.ilog10() + 1);

                if result % j == *i {
                    Some(result / j)
                } else {
                    None
                }
            } else {
                None
            },
        ]
        .iter()
        .flatten()
        .any(|i| _d7recurse(*i, operand, next, day2))
    } else {
        println!("{} ?= {}", operand, result);
        operand == result
    }
}

fn _d7p1(file: String) -> u64 {
    let lines: Vec<&str> = file.lines().collect();
    let mut sum: u64 = 0;

    for line in lines {
        let result = line.split(':').nth(0).unwrap().parse::<u64>().unwrap();
        let operands: Vec<u64> = line
            .split(':')
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        println!("{}: {:?}", result, operands);
        if _d7recurse(result, operands[0], &operands[1..], false) {
            sum += result;
        }
    }

    return sum;
}

fn _d7p2(file: String) -> u64 {
    let lines: Vec<&str> = file.lines().collect();
    let mut sum: u64 = 0;

    for line in lines {
        let result = line.split(':').nth(0).unwrap().parse::<u64>().unwrap();
        let operands: Vec<u64> = line
            .split(':')
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        println!("{}: {:?}", result, operands);
        if _d7recurse(result, operands[0], &operands[1..], true) {
            sum += result;
        }
    }

    return sum;
}

fn _d8p1(file: String) -> i32 {
    let charv: Vec<char> = file.chars().collect();
    let upperbound = charv.len() as i32;
    let linelen: i32 = file.lines().nth(0).unwrap().len() as i32 + 2;
    let mut antinodes: HashSet<i32> = HashSet::new();
    println!("Line len: {}",linelen);
    let mut sum: i32 = 0;
    let mut chars: HashSet<char> = HashSet::new();
    for char in charv.iter() {
        if *char != '.' && *char != '\n' && *char != '\r' {
            chars.insert(*char);
        }
    }
    for char in chars {
        let mut locations: Vec<i32> = Vec::new();
        charv.iter().enumerate().for_each(|(pos, c)| {
            if *c == char {
                locations.push(pos as i32);
            }
        });
        // println!("For character '{}', locations are:", char); 
        // println!("{:?}", locations);
        // stdin().read(&mut [0]).unwrap();

        for loc in &locations {
            for pair in &locations {
                let first =  cmp::min(loc,&pair);
                let second = cmp::max(loc,&pair);
                let dist = (loc-pair).abs();

                if second % linelen > first % linelen 
                // direction is NW-SE
                    && (second + dist) % linelen > second % linelen
                // antinode to SE is not OOB
                    && second + dist >= 0
                    && (second + dist) % linelen < 50
                    && second + dist < upperbound
                {
                    antinodes.insert(second + dist);
                } if second % linelen > first % linelen
                // direction is NW-SE
                    && (first - dist) % linelen <= first % linelen
                // antinode to NW is not OOB
                    && first - dist >= 0
                    && (first - dist) % linelen < 50
                    && first - dist < upperbound
                {
                    antinodes.insert(first - dist);
                } if second % linelen < first % linelen
                // direction is SW-NE
                    && (second + dist) % linelen < second % linelen
                // antinode to SW is not OOB
                    && second + dist >= 0
                    && (second + dist) % linelen < 50
                    && second + dist < upperbound
                {
                    antinodes.insert(second + dist);
                } if second % linelen < first % linelen
                //direction is SW-NE
                    && (first - dist) % linelen >= first % linelen
                // antinode to NE is not OOB
                    && first - dist >= 0
                    && (first - dist) % linelen < 50
                    && first - dist < upperbound
                {
                    antinodes.insert(first - dist);
                }
            }
        }
        // let mut state= charv.clone();
        // for i in antinodes {
        //     state[i as usize] = '@';
        // }
        // println!("For character '{}', antinodes are:", char); 
        // println!("{}", state.iter().collect::<String>());
        // stdin().read(&mut [0]).unwrap();
    }
    sum += antinodes.len() as i32;
    return sum;

}

fn _d8p2(file: String) -> i32 {
    let charv: Vec<char> = file.chars().collect();
    let upperbound = charv.len() as i32;
    let linelen: i32 = file.lines().nth(0).unwrap().len() as i32 + 2;
    let mut antinodes: HashSet<i32> = HashSet::new();
    println!("Line len: {}",linelen);
    let mut sum: i32 = 0;
    let mut chars: HashSet<char> = HashSet::new();
    for char in charv.iter() {
        if *char != '.' && *char != '\n' && *char != '\r' {
            chars.insert(*char);
        }
    }
    for char in chars {
        let mut locations: Vec<i32> = Vec::new();
        charv.iter().enumerate().for_each(|(pos, c)| {
            if *c == char {
                locations.push(pos as i32);
            }
        });
        // println!("For character '{}', locations are:", char); 
        // println!("{:?}", locations);
        // stdin().read(&mut [0]).unwrap();

        for loc in &locations {
            antinodes.insert(*loc); //spent 2 hours for this.
            for pair in &locations {
                let first =  cmp::min(loc,&pair);
                let second = cmp::max(loc,&pair);
                let dist = (loc-pair).abs();
                for i in 1..75 {

                    if second % linelen > first % linelen
                    // direction is NW-SE
                        && (second + dist*i) % linelen > second % linelen
                    // antinode to SE is not OOB
                        && second + dist*i >= 0
                        && (second + dist*i) % linelen < linelen-2
                        && second + dist*i < upperbound
                    {
                       antinodes.insert(second + dist*i);
                    } else {break};
                }
                for i in 1..50 {
                    if second % linelen > first % linelen
                    // direction is NW-SE
                        && (first - dist*i) % linelen <= first % linelen
                    // antinode to NW is not OOB
                        && first - dist*i >= 0
                        && (first - dist*i) % linelen < linelen-2
                        && first - dist*i < upperbound
                    {
                        antinodes.insert(first - dist*i);
                    }else {break};
                }


                for i in 1..50 {
                    if second % linelen < first % linelen
                    // direction is SW-NE
                        && (second + dist*i) % linelen < second % linelen
                    // antinode to SW is not OOB
                        && second + dist*i >= 0
                        && (second + dist*i) % linelen < linelen-2
                        && second + dist*i < upperbound
                    {
                        antinodes.insert(second + dist*i);
                    }else {break};
                }
                for i in 1..50 {
                    if second % linelen < first % linelen
                    //direction is SW-NE
                        && (first - dist*i) % linelen >= first % linelen
                    // antinode to NE is not OOB
                        && first - dist*i >= 0
                        && (first - dist*i) % linelen < linelen-2
                        && first - dist*i < upperbound
                    {
                        antinodes.insert(first - dist*i);
                    }else {break};
                }

            }
        }
        // let mut state= charv.clone();
        // for i in &antinodes {
        //     state[*i as usize] = '@';
        // }
        // println!("For character '{}', antinodes added:", char); 
        // println!("{}", state.iter().collect::<String>());
        // stdin().read(&mut [0]).unwrap();
    }
    sum += antinodes.len() as i32;
    return sum;

}
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = read_file(file_path);
    let ret = _d8p2(file);
    println!("{}", ret);
}
