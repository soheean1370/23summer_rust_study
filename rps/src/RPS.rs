use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력을 읽을 수 없습니다.");

    let items: Vec<&str> = input.trim().split_whitespace().collect();
    let mut result = String::new();

    // 리스트 정렬
    let mut sorted_items = items.clone();
    sorted_items.sort();

    if sorted_items[0] == sorted_items[1] {
        result = "D".to_string();
    } else if sorted_items[0] == "P" {
        if sorted_items[1] == "R" {
            result = "P".to_string();
        } else {
            result = "S".to_string();
        }
    } else {
        result = "R".to_string();
    }

    println!("{}", result);
}


