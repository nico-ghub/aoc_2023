use std::fs::read_to_string;

pub fn part1(){
    let mut total = 0;
    for line in read_to_string( "./src/day1/input_p1.txt" ).unwrap().lines(){
        for first in line.chars() {
            if first.is_numeric(){
                for last in line.chars().rev() {
                    if last.is_numeric(){
                        total+=format!("{first}{last}").parse::<i32>().unwrap();
                        break;
                    }
                }
                break;
            }
        }
    }
    println!("day1 part1; {total}");
}

use std::collections::HashMap;

pub fn part2(){

    let m: HashMap<&str, i32> = HashMap::from([
        ("zero",  0),
        ("one",   1),
        ("two",   2),
        ("three", 3),
        ("four",  4),
        ("five",  5),
        ("six",   6),
        ("seven", 7),
        ("eight", 8),
        ("nine",  9),
        ("0",     0),
        ("1",     1),
        ("2",     2),
        ("3",     3),
        ("4",     4),
        ("5",     5),
        ("6",     6),
        ("7",     7),
        ("8",     8),
        ("9",     9),
    ]);

    fn search( line: &str, m: &HashMap<&str, i32>, range: Vec<usize> ) -> i32 {
        for i in range{
            for (k, v) in m {
                if ( i+k.len()<=line.len() ) && ( line[i..i+k.len()] == **k ) {
                    return *v;
                }
            }
        }
        panic!("Nothing found in {line}"); 
    }

    let mut total = 0;
    for line in read_to_string( "./src/day1/input_p2.txt" ).unwrap().lines(){
        let first = search( line, &m, (0..line.len()).collect::<Vec<usize>>() );
        let last  = search( line, &m, (0..line.len()).rev().collect::<Vec<usize>>() );
        total+=format!("{first}{last}").parse::<i32>().unwrap();
    }
    println!("day1 part2; {total}");
}