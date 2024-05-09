use std::io;

fn check_rectangle() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let l: Vec<u32> = input.trim().split_whitespace().map(|x| x.parse().expect("")).collect();

    let l_x = &l.iter().copied().step_by(2).collect::<Vec<u32>>();
    let l_y = &l.iter().skip(1).copied().step_by(2).collect::<Vec<u32>>();

    let x_max = u32::max(l_x[1], l_x[3]) - u32::min(l_x[0], l_x[2]);
    let y_max = u32::max(l_y[1], l_y[3]) - u32::min(l_y[0], l_y[2]);

    let mut x1 = l_x[0];
    let mut x2 = l_x[1];
    let mut x3 = l_x[2];
    let mut x4 = l_x[3];
    let mut y1 = l_y[0];
    let mut y2 = l_y[1];
    let mut y3 = l_y[2];
    let mut y4 = l_y[3];

    if x3 < x1 {
        let temp = x1;
        x1 = x3;
        x3 = temp;
        let temp = x2;
        x2 = x4;
        x4 = temp;
    }

    if x_max < (x2 - x1) + (x4 - x3) && y_max < (y2 - y1) + (y4-y3) {
        println!("FACE");
    } else if x_max == (x2 - x1) + (x4 - x3) && y_max == (y2 - y1)+(y4-y3) {
        println!("POINT");
    } else if (x_max == (x2 - x1) + (x4 - x3) && y_max <= (y2 - y1)+(y4-y3))
        || (x_max <= (x2 - x1) + (x4 - x3) && y_max == (y2 - y1)+(y4-y3))
    {
        println!("LINE");
    } else {
        println!("NULL");
    }
}

fn main() {
    for _ in 0..4 {
        check_rectangle();
    }
}
