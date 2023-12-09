use std::{fs::read_to_string, collections::HashMap};
use  num::integer::lcm;

type NodeMapT = HashMap<String,(String,String)>;

fn load_input( path: &str ) -> ( Vec<char>, NodeMapT ){
    let lines = read_to_string( path ).unwrap();
    let mut lines =lines.lines();
    let p: Vec<char> = lines.next().unwrap().chars().collect::<Vec<_>>();
    lines.next();
    ( p, lines.into_iter().map( |l| (l[0..3].to_string(), (l[7..10].to_string(), l[12..15].to_string())) ).collect::<NodeMapT>() )
} 

pub fn part1(){
    let mut result = 0;
    let path: &str = "./src/day8/input_p1.txt";
    // let path = "./src/day8/example_p1.txt";

    let (p, map) = load_input( path );
    let mut n = "AAA";
    while *n != *"ZZZ" {
        n = if p[ result % p.len() ] == 'L' { &map[n].0 } else { &map[n].1 };
        result+=1;
    }
    println!("day8 part1: {result}" );
}

pub fn part2(){
    let mut result = 1;
    let path = "./src/day8/input_p1.txt";
    // let path = "./src/day8/example_p2.txt";

    let (p, map) = load_input( path );
    for ( mut n, _ ) in &map {
        if n.ends_with("A"){
            let mut i = 0;
            while ! n.ends_with("Z") {
                n = if p[ i % p.len() ] == 'L' { &map[n].0 } else { &map[n].1 };
                i+=1;
            }
            result = lcm( result, i);
        }
    }
    println!("day8 part2: {result}" );
}