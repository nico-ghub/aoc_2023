use std::{fs::read_to_string, collections::HashMap};

fn count_char(s: &str, c: char ) -> i32 {
    s.chars().filter( |c2| *c2==c ).count() as i32
}

fn hand_value( hand: &str ) -> i32{
    hand.chars().map( |c| count_char( hand, c ) ).into_iter().sum()
}

pub fn part1(){
    let mut result = 0;
    let path = "./src/day7/input_p1.txt";
    // let path = "./src/day7/example_p1.txt";
    
    fn hand_value( hand: &str ) -> i32{
        hand.chars().map( |c| count_char( hand, c ) ).into_iter().sum()
    }

    let m: HashMap<char, usize> = "23456789TJQKA".chars().enumerate().map( |(a,b)| (b,a) ).collect();
    let mut hands: Vec<(i32, Vec<usize>,i32)> = vec![];
    
    for line in read_to_string( path ).unwrap().lines(){
        let (a, b) = line.split_once( " " ).unwrap();
        hands.push( (hand_value(a), a.chars().map(|c| m[&c] ).collect::<Vec<_>>(), b.parse().unwrap() ) );
    }
    hands.sort();
    result = hands.iter().enumerate( ).map( |( i, (_,_,bid)  )| (i+1) as i32*bid ).sum();

    println!("day7 part1: {result}");
}

pub fn part2(){
    let mut result = 0;
    let path = "./src/day7/input_p1.txt";
    // let path = "./src/day7/example_p1.txt";

    let m: HashMap<char, usize> = "J23456789TQKA".chars().enumerate().map( |(a,b)| (b,a) ).collect();
    let mut hands: Vec<(i32, Vec<usize>,i32)> = vec![];

    fn hand_value_J( hand: &str ) -> i32{
        "J23456789TQKA".chars().map( | c| hand_value( &(hand.replace( 'J', &c.to_string() ))[..] ) ).max().unwrap()
    }

    for line in read_to_string( path ).unwrap().lines(){
        let (a, b) = line.split_once( " " ).unwrap();
        hands.push( (hand_value_J(a), a.chars().map(|c| m[&c] ).collect::<Vec<_>>(), b.parse().unwrap() ) );
    }
    hands.sort();
    result = hands.iter().enumerate( ).map( |( i, (_,_,bid)  )| (i+1) as i32*bid ).sum();

    println!("day7 part2: {result}");
}