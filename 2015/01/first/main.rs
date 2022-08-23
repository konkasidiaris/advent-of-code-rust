use std::fs;

fn main() {
    let contents = fs::read_to_string("./2015/01/first/input.txt").expect("there should be a file");
    let mut i = 0;
    for char in contents.chars() {
        if char == '(' {
            i += 1;
        } else {
            i -= 1;
        }
    }
    println!("{}", i);
}
