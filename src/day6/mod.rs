fn solve( part:i32, data: Vec<(i64, i64)> ){
    let mut result=1;
    for (t,d) in data{
        for i in 1..t/2 {
            if i * ( t - i ) > d {
                result *= t+1 - 2*i;
                break
            }
        }
    }
    println!("day6 part{part}: {result}");
}

pub fn part1(){
    solve( 1, vec![(62,644),(73,1023),(75,1240),(65,1023)] );
}

pub fn part2(){
    solve( 2, vec![(62737565, 644102312401023 as i64)] );
}