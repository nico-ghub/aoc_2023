use std::fs::read_to_string;
use std::cmp;

pub fn part1(){
    let mut part1 = 0;
    let mut part2 = 0;
    let path = "./src/day2/input_p1.txt";
    // let path = "./src/day2/example_p1.txt";
    for line in read_to_string( path ).unwrap().lines(){
        if let Some((game, data))= line.split_once(":"){
            if let Some((_, game))=game.split_once(" "){
                let game:i32 = game.parse().unwrap();
                let mut r =0;
                let mut g=0;
                let mut b=0;                
                for set in data.split( ";" ){
                    for cube in set.split(","){
                        if let Some( (n, color) ) = cube.trim().split_once(" "){
                            let n = n.parse().unwrap();
                            if color == "red" {
                                r = cmp::max( r, n );
                            } else if color == "green" {
                                g = cmp::max( g, n );
                            } else if color == "blue"{
                                b = cmp::max( b, n );
                            }
                        }
                    }
                }
                if r <= 12 && g <= 13 && b <= 14 {
                    part1+=game;
                }
                part2+= r*g*b;
            }
        }
    }
    println!("day2 part1: {part1}");
    println!("day2 part2: {part2}");
}


pub fn part2(){
}