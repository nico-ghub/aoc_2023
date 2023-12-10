use std::fs::read_to_string;

type GridT = Vec<Vec<(char,bool)>>;

fn find_start( grid: &GridT ) -> ( usize, usize, char, char ){
    for (y, l) in grid.iter().enumerate( ){
        for (x, (c,_)) in l.iter().enumerate( ){
            if *c == 'S' {
                let left = x > 0             && ( "FL-".contains( grid[y][x-1].0 ) );
                let right= x < grid[0].len() && ( "J7-".contains( grid[y][x+1].0 ) );
                let up   = y > 0             && ( "F7|".contains( grid[y-1][x].0 ) );
                let down = y < grid.len()    && ( "JL|".contains( grid[y+1][x].0 ) );
                let (start,direction) = match (up, down, left, right) {
                    (true,  true,  false, false ) => ('|','^'),
                    (true,  false, true,  false ) => ('J','^'),
                    (true,  false, false, true  ) => ('L','^'),
                    (false, true,  true,  false ) => ('7','v'),
                    (false, true,  false, true  ) => ('F','v'),
                    (false, false, true,  true  ) => ('-','>'),
                    _ => panic!("{up} {down} {left} {right}"),
                };
                return (x,y,start,direction);
            }
        }
    }
    panic!("Failed to find start position");
}

pub fn part1(){
    let path = "./src/day10/input_p1.txt";
    // let path = "./src/day10/example_p1.txt";
    // let path = "./src/day10/example_p2.txt";

    let mut grid: GridT = read_to_string( path ).unwrap().lines().map( |x| x.chars().map(|c|(c,false)).collect() ).collect();
    let (mut x, mut y, start, mut direction ) = find_start( &grid );
    let mut part1 = 0;

    grid[y][x].0 = start;
    
    loop {
        grid[y][x].1=true;
        part1+=1;
        match direction {
            '>' => x+=1,
            '<' => x-=1,
            '^' => y-=1,
            'v' => y+=1,
            _ => panic!("")
        };
        if grid[y][x].1 {
            break;
        }
        direction = match ( direction, grid[y][x].0 ) {
            ('>', '-') => '>',
            ('<', '-') => '<',
            ('^', '|') => '^',
            ('v', '|') => 'v',
            ('>', '7') => 'v',
            ('^', '7') => '<',
            ('>', 'J') => '^',
            ('v', 'J') => '<',
            ('v', 'L') => '>',
            ('<', 'L') => '^',
            ('^', 'F') => '>',
            ('<', 'F') => 'v',
            _ => panic!( "impossibru" )
        };
    }
    println!("day10 part1: {}", part1/2);
    
    let mut part2=0;
    for (_, l) in grid.iter().enumerate( ){
        let mut dedans=false;
        let mut current = '.';
        for (_, (c, v)) in l.iter().enumerate( ){
            if *v {
                if "LF".contains(*c){
                    current = *c;
                } else if ( *c == '|' ) ||
                          ( ( *c == 'J' ) && ( current == 'F' ) ) ||
                          ( ( *c == '7' ) && ( current == 'L' ) ) {
                    dedans = ! dedans;
                }
            } else {
                if dedans {
                    part2+=1;
                }
            }
        }
    }
    println!("day10 part2: {part2}");
}

pub fn part2(){
}