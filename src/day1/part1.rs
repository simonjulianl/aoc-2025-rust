use std::fs;

fn main() {
    let file_path = "input.txt";

    // Attempt to read the file contents
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut dial: i64 = 50;
    let mut ans = 0;
    for line in contents.lines() {
        let (prefix, number_str) = line.split_at(1);
        let number = number_str.parse::<i64>().unwrap();
        if prefix == "L" {
            dial -= number;
        } else {
            dial += number;
        }

        dial %= 100;

        if dial == 0 {
            ans = ans + 1;
        }
    }

    println!("ans={ans}")
}
