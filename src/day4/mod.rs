use std::fs::read_to_string;
use std::collections::HashSet;

pub fn part1(){
    let mut part1 = 0;
    let mut part2 = 0;
    let path = "./src/day4/input_p1.txt";
    // let path = "./src/day4/example_p1.txt";
    let mut cards = vec![0; 216];
    for (card, line) in read_to_string( path ).unwrap().lines().enumerate(){
        cards[card]+=1;
        if let Some((_, data))= line.split_once(":"){
            if let Some((winnings, numbers))= data.split_once("|"){
                let winnings=winnings.split_whitespace().collect::<HashSet<_>>();
                let numbers=numbers.split_whitespace().collect::<HashSet<_>>();
                let m=winnings.intersection(&numbers).collect::<Vec<_>>().len();
                if m > 0{
                    part1+=1 << (m-1);
                    for i in 0..m{
                        cards[card+i+1] += cards[card];
                    }
                }
                part2+=cards[card];
            }
        }
    }
    println!("day4 part1: {part1}");
    println!("day4 part2: {part2}");   
}

pub fn part2(){
}