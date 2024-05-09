use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력을 읽을 수 없습니다.");

    let items: Vec<&str> = input.trim().split("").collect();
    let mut plus = 1;
    let mut result = 0;

    for i in items{
        if i == "o"{
            result -= 1;
            plus = 1;
        }
        else if i == "h"{
            result += plus;
            plus += 1;
        }
    }
    println!("{}",result);

}