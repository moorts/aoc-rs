use std::fmt::Display;

fn is_safe(report: &Vec<i32>) -> bool {
    if report.len() <= 1 { 
        return true;
    }

    let is_increasing = report[1] > report[0];

    for i in 0..report.len()-1 {
        if is_increasing && report[i] >= report[i+1] {
            return false;
        }

        if !is_increasing && report[i] <= report[i+1] {
            return false;
        }

        let diff = report[i].abs_diff(report[i+1]);

        if diff == 0 || diff > 3 {
            return false;
        }
    }

    true
}

pub fn part1(input: &str) -> impl Display {
    let lines = input.lines();

    let mut reports: Vec<Vec<i32>> = Vec::new();

    for report in lines {
        let levels = report.split_ascii_whitespace().map(|level| level.parse::<i32>().unwrap()).collect();

        reports.push(levels);
    }
    
    reports.into_iter().map(|report| if is_safe(&report) { 1 } else { 0 }).sum::<i32>()
}

fn is_safe_removed(report: &Vec<i32>, exclude: usize) -> bool {
    let is_increasing = if exclude == 0 {
        report[1] < report[2]
    } else if exclude == 1 {
        report[0] < report[2]
    } else {
        report[0] < report[1]
    };

    for i in 0..report.len()-2 {
        let mut idx = i;
        let mut next_idx = i+1;

        if i >= exclude {
            idx += 1;
            next_idx += 1;
        } else if i + 1 == exclude {
            next_idx += 1;
        }

        let dist = report[next_idx] - report[idx];

        if is_increasing && (dist < 1 || dist > 3) {
            return false;
        }

        if !is_increasing && (dist < -3 || dist > -1) {
            return false;
        }
    }
    
    true
}

fn is_safe2(report: &Vec<i32>, increasing: bool) -> bool {
    for i in 0..report.len() - 1 {
        let dist = if increasing { report[i+1] - report[i] } else { report[i] - report[i+1] };
        if dist < 1 || dist > 3 {
            return is_safe_removed(&report, i) || is_safe_removed(&report, i+1);
        }
    }

    true
}

pub fn part2(input: &str) -> impl Display {
    let lines = input.lines();

    let mut reports: Vec<Vec<i32>> = Vec::new();

    for report in lines {
        let levels = report.split_ascii_whitespace().map(|level| level.parse::<i32>().unwrap()).collect();

        reports.push(levels);
    }
    
    reports.into_iter().map(|report| if is_safe2(&report, true) || is_safe2(&report, false) { 1 } else { 0 }).sum::<i32>()
}
