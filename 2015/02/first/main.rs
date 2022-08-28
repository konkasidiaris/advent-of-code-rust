use std::fs;

fn main() {
    let contents = fs::read_to_string("./2015/02/first/input.txt").expect("there should be a file");
    let mut total_area: i32 = 0;

    for line in contents.lines() {
        let mut vec: Vec<i32> = line.split("x").map(|x| x.parse::<i32>().unwrap()).collect();
        vec.sort();
        total_area = total_area + 3 * vec[0] * vec[1] + 2 * vec[0] * vec[2] + 2 * vec[1] * vec[2]
    }

    println!("{}", total_area);
}
