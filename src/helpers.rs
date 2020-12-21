use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::string::String;

const ERRUID: i32 = -1;

pub fn get_uid(args: &[String]) -> i32 {
    if args.len() > 2 {
        let custom = args[2].parse().unwrap_or_else(|_| {
            println!("failed to parse UID from input. Use default UID");
            0
        });
        std::cmp::max(custom, 0)
    } else {
        println!("Use default UID");
        0
    }
}

pub fn get_rules_from_file(filename: &str) -> Vec<String> {
    let rules = if let Ok(file) = File::open(filename) {
        let lines = io::BufReader::new(file).lines();
        lines.fold(Vec::<String>::new(), |mut acc, s| {
            if let Ok(v) = s {
                if v.len() > 4 {
                    acc.push(normalize(&v));
                }
            }
            acc
        })
    } else {
        Vec::<String>::new()
    };

    if !rules.is_empty() {
        rules
    } else {
        println!("failed to parse rules file {}. Use default rules", filename);
        get_default_rules()
    }
}

pub fn get_rules(args: &[String]) -> Vec<String> {
    if args.len() > 1 {
        get_rules_from_file(&args[1])
    } else {
        println!("Use default rules");
        get_default_rules()
    }
}

pub fn uid_from_str(s: &str, skip: usize) -> i32 {
    i32::from_str(&s[skip..]).unwrap_or(ERRUID)
}

fn normalize(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn get_default_rules() -> Vec<String> {
    let default: Vec<&'static str> = vec![
        "uid==3",
        "uid>3",
        "uid>101",
        "uid<3",
        "uid<101",
        "uid==3|uid>3",
        "uid==3|uid<3",
        "uid==3|uid==4|uid==5",
        "uid==3&uid==4&uid==5",
        "(uid==3|uid==4)&uid==5",
        "(uid==3&uid==4)|uid==5",
        "(uid==3|uid<3)|uid==100",
        "((uid==3|uid<3))|uid==100",
        "(uid==3  |  uid<5) | uid==100",
        "(uid>50 & uid<100) & (uid>40 & uid<60)",
        "((uid>50 & uid<100) & (uid>40 & uid<60))|(uid==101|uid==102|uid==103)",
        "uid=3|uid*4",
        "uid=zzz|uid*4",
    ];

    default.iter().fold(Vec::<String>::new(), |mut a, s| {
        a.push(normalize(s));
        a
    })
}
