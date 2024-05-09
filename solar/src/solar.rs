use std::io;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("");
    let m: usize = iter.next().unwrap().parse().expect("");

    let mut arr: Vec<Vec<usize>> = vec![vec![0; m]; n];
    let mut data: HashMap<(usize, usize), usize> = HashMap::new();

    let dx = [1, 0, -1, 0];
    let dy = [0, 1, 0, -1];
    let mut x = 0;
    let mut y = 0;
    let mut num = 0;
    let mut direction = 0;

    while num < n * m {
        num += 1;
        arr[x][y] = num;
        let coordinate = (x + 1, y + 1);
        data.insert(coordinate, num);

        let nx = x as isize + dx[direction];
        let ny = y as isize + dy[direction];

        if 0 <= nx && nx < n as isize && 0 <= ny && ny < m as isize && arr[nx as usize][ny as usize] == 0 {
            x = nx as usize;
            y = ny as usize;
        } else {
            direction = (direction + 1) % 4;
            x = (x as isize + dx[direction]) as usize;
            y = (y as isize + dy[direction]) as usize;
        }
    }

    for _ in 0..4 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("");
        let values: Vec<usize> = input.trim().split_whitespace().map(|x| x.parse().expect("")).collect();

        if values.len() == 2 {
            let (x_coord, y_coord) = (values[1], values[0]);
            if x_coord <= n && y_coord <= m {
                println!("{}", data.get(&(x_coord, y_coord)).cloned().unwrap_or(0));
            } else {
                println!("0");
            }
        } else {
            let cell_number = values[0];
            if cell_number > n * m {
                println!("0 0");
            } else {
                match data.iter().find(|&(_, &v)| v == cell_number) {
                    Some((&(x, y), _)) => println!("{} {}", y, x),
                    None => println!("0 0"),
                }
            }
        }
    }
}
