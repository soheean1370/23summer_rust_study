use std::collections::HashSet;
use std::io;

fn main() {
    let RPS: Vec<&str> = vec!["R", "P", "S", "D"];
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력을 읽을 수 없습니다.");

    let items: Vec<&str> = input.trim().split_whitespace().collect();

    let items_set: HashSet<&str> = items.iter().cloned().collect();
    let mut sorted_items: Vec<&str> = items_set.into_iter().collect();
    sorted_items.sort();


    if sorted_items.len() == 1 || sorted_items.len() == 3 {
        println!("{}", RPS[3]);
    } else if !sorted_items.contains(&"R") {
        println!("{}", RPS[2]);
    } else if !sorted_items.contains(&"P") {
        println!("{}", RPS[0]);
    } else if !sorted_items.contains(&"S") {
        println!("{}", RPS[1]);
    }
}
