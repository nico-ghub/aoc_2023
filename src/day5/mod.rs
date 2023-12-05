use std::fs::read_to_string;

fn string_to_int_vec( s: &str ) -> Vec<i64> {
    s.split_whitespace().map( |n| n.parse().unwrap() ).collect::<Vec<i64>>()
}

fn load(path :&str) -> (  Vec<i64>, Vec<Vec<Vec<i64>>> ){
    let lines = read_to_string( path ).unwrap();
    let mut lines =lines.lines();

    let seeds: Vec<i64> = string_to_int_vec( lines.next().unwrap().split_once(":").unwrap().1 );
    lines.next();
    lines.next();

    let mut maps: Vec<Vec<Vec<i64>>> = vec![];
    {
        let mut current_map: Vec<Vec<i64>> = vec![];
        for line in lines {
            if line == "" {
                continue;
            } 
            if line.contains(":"){
                current_map.sort_by(|a,b| a[1].cmp( &b[1] ) );
                maps.push( current_map );
                current_map=vec![];
                continue;
            }
            current_map.push( string_to_int_vec( line ) );
        }

        current_map.sort_by(|a,b| a[1].cmp( &b[1] ) );
        maps.push( current_map );


    }
    (seeds, maps)
}

pub fn part1(){
    let mut result = std::i64::MAX;
    let path = "./src/day5/input_p1.txt";
    // let path = "./src/day5/example_p1.txt";

    let (seeds, maps) = load( path );

    for seed in seeds{
        let mut x= seed;
        for map in &maps{
            for line in map{
                if x >= line[1] && x <= line[1] + line[2]{
                    x = x - line[1] + line[0];
                    break;
                }
            }
        }
        if x < result {
            result = x;
        }
    }
    println!("day5 part1: {result}");
}

use std::cmp;

pub fn part2(){
    let mut result = std::i64::MAX;
    let path = "./src/day5/input_p1.txt";
    // let path = "./src/day5/example_p1.txt";

    let (seeds, maps) = load( path );

    let mut range: Vec<(i64,i64)> = vec![];
    for i in 0..seeds.len()/2{
        range.push( (seeds[2*i], seeds[2*i+1]) );
    }

    for map in maps{
        let mut new_range: Vec<(i64,i64)> = vec![];
        for (mut from, mut len) in range.clone() {
            for line in &map{
                if from >= line[1] && from <= line[1] + line[2]{
                    let size = cmp::min( len, line[2]- ( from-line[1] ) );
                    new_range.push( ( from - line[1] + line[0], size) );
                    from += size;
                    len -=size;
                    if len == 0 {
                        break;
                    }
                }
            }
            if len > 0 {
                new_range.push( ( from, len ) );;
            }
        }
        range = new_range;
    }

    for (from, _ ) in range {
        if from < result {
            result = from;
        }
    }

    println!("day5 part2: {result}");
}