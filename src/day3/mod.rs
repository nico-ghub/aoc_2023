use std::collections::HashMap;
use std::collections::HashSet;

use std::fs::read_to_string;
use std::cmp;

pub fn part1(){
    let mut part1 = 0;
    let mut part2 = 0;

    let path = "./src/day3/input_p1.txt";
    // let path = "./src/day3/example_p1.txt";
    let mut grid: Vec<Vec<char>> = vec![];
    for line in read_to_string( path ).unwrap().lines(){
        let line: Vec<char> = line.chars().collect();
        grid.push( line );
    }
    
    let xmax=grid.len()-1;
    let ymax=grid[0].len()-1;

    let mut number=0;
    let mut is_part = false;
    
    let mut all_gears: HashMap<(usize,usize), i32> = HashMap::new();
    let mut gears= HashSet::new();

    for (x, line) in grid.iter().enumerate( ){
        for (y, c) in line.iter().enumerate( ){
            if c.is_numeric(){
                number = number * 10 + *c as i32 - '0' as i32;
                
                let imin= cmp::max( x as i32 -1, 0 ) as usize;
                let imax= cmp::min( x+1, xmax ) + 1;
                let jmin= cmp::max( y as i32 -1, 0 ) as usize;
                let jmax= cmp::min( y+1, ymax ) + 1;

                for i in imin..imax {
                    for j in jmin..jmax {
                        let c2=grid[i][j];
                        if ! c2.is_numeric() && c2 != '.' {
                            if c2 == '*'{
                                gears.insert((i,j));
                            }
                            is_part = true;
                        }
                    }
                }
            }
            
            if ( ! c.is_numeric() ) || (y == ymax)  {
                if number != 0{
                    if is_part {
                        part1+=number;
                        is_part=false;
                    }
                    for g in gears.iter(){
                        if all_gears.contains_key(g){
                            part2 += number*all_gears[g];
                        } else {
                            all_gears.insert(*g, number );
                        }
                    }
                    gears.clear();

                    number = 0;
                }
            }
        }
    }
    println!("day3 part1: {part1}");
    println!("day3 part2: {part2}");
}


pub fn part2(){
}