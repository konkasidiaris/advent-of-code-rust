use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./2015/02/second/input.txt").expect("there should be a file");
    let mut total_length: i32 = 0;

    for line in contents.lines() {
        let mut vec: Vec<i32> = line.split("x").map(|x| x.parse::<i32>().unwrap()).collect();
        vec.sort();
        total_length = total_length + 2 * vec[0] + 2 * vec[1] + vec[0] * vec[1] * vec[2]
    }

    println!("{}", total_length);
}
