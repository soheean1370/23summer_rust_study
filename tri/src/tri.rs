use std::io;
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let TRI: Vec<char> = vec!['0', '1', '2', '3'];
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력을 읽을수 없습니다");

    let items: Vec<u32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse())
        .collect::<Result<Vec<u32>, ParseIntError>>()?;

    let a = items[0];
    let b = items[1];
    let c = items[2];

    if (a >= b + c) || (b >= c + a) || (c >= a + b) {
        println!("{}", TRI[0]);
    } else {
        if (a.pow(2) == b.pow(2) + c.pow(2))
            || (b.pow(2) == c.pow(2) + a.pow(2))
            || (c.pow(2) == a.pow(2) + b.pow(2))
        {
            println!("{}", TRI[1]);
        } else if (a.pow(2) > b.pow(2) + c.pow(2))
            || (b.pow(2) > c.pow(2) + a.pow(2))
            || (c.pow(2) > a.pow(2) + b.pow(2))
        {
            println!("{}", TRI[2]);
        } else if (a.pow(2) < b.pow(2) + c.pow(2))
            || (b.pow(2) < c.pow(2) + a.pow(2))
            || (c.pow(2) < a.pow(2) + b.pow(2))
        {
            println!("{}", TRI[3]);
        }
    }

    Ok(())
}
