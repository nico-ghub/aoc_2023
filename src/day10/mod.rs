use std::fs::read_to_string;

pub fn part1(){
    let path = "./src/day10/input_p1.txt";
    // let path = "./src/day10/example_p1.txt";
    // let path = "./src/day10/example_p2.txt";

    let mut grid: Vec<Vec<char>> = read_to_string( path ).unwrap().lines().map( |x| x.chars().collect::<Vec<_>>() ).collect::<Vec<_>>();
    let mut boucle: Vec<(usize,usize)> = vec![];
    let mut direction: char = '0';

    for (y, l) in grid.clone().iter().enumerate( ){
        for (x, c) in l.iter().enumerate( ){
            if *c == 'S' {
                boucle.push((x,y));
                let left = x > 0             && ( "FL-".contains( grid[y][x-1] ) );
                let right= x < grid[0].len() && ( "J7-".contains( grid[y][x+1] ) );
                let up   = y > 0             && ( "F7|".contains( grid[y-1][x] ) );
                let down = y < grid.len()    && ( "JL|".contains( grid[y+1][x] ) );
                (grid[y][x],direction) = match (up, down, left, right) {
                    (true,  true,  false, false ) => ('|','^'),
                    (true,  false, true,  false ) => ('J','^'),
                    (true,  false, false, true  ) => ('L','^'),
                    (false, true,  true,  false ) => ('7','v'),
                    (false, true,  false, true  ) => ('F','v'),
                    (false, false, true,  true  ) => ('-','>'),
                    _ => panic!("{up} {down} {left} {right}"),
                };
            }
        }
    }

    loop {
        let (mut x,mut y) = boucle.last().unwrap().clone();
        match direction {
            '>' => x+=1,
            '<' => x-=1,
            '^' => y-=1,
            'v' => y+=1,
            _ => panic!("")
        };
        if (x,y) == boucle[0] {
            break;
        }
        boucle.push( (x,y) );
        direction = match ( direction, grid[y][x] ) {
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
    println!("day10 part1: {}", boucle.len()/2);
    
    let mut part2=0;
    for (y, l) in grid.iter().enumerate( ){
        let mut dedans=false;
        let mut current = '.';
        for (x, c) in l.iter().enumerate( ){
            if boucle.contains( &(x,y) ){
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