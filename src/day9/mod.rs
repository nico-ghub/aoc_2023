use std::fs::read_to_string;

pub fn part1(){
    let mut result: i64 = 0;
    let path = "./src/day9/input_p1.txt";
    // let path = "./src/day9/example_p1.txt";

    for line in read_to_string( path ).unwrap().lines(){
        let mut data: Vec<i64> = line.split_whitespace().map( |x| x.parse::<i64>().unwrap() ).collect::<Vec<_>>();
        while data.len() > 0 {
            for i in 1..data.len() {
                data[i-1] = data[i] - data[i-1];
            }
            result += data.pop().unwrap();
        }
    }
    println!("day9 part1: {result}");
}

pub fn part2(){
    let mut result = 0;
    let path = "./src/day9/input_p1.txt";
    // let path = "./src/day9/example_p1.txt";

    for line in read_to_string( path ).unwrap().lines(){
        let mut data: Vec<i64> = line.split_whitespace().rev().map( |x| x.parse::<i64>().unwrap() ).collect::<Vec<_>>();
        let mut x=1;
        while data.len() > 0 {
            for i in 0..data.len()-1 {
                data[i] = data[i] - data[i+1];
            }
            result+=x*data.pop().unwrap();
            x*=-1;
        }
    }
    println!("day9 part2: {result}");
}