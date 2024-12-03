use regex::Regex;
use std::env;
use std::fs::File;
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
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = read_file(file_path);
    let ret = _d3p2(file);
    println!("{}", ret);
}
