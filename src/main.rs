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

        let total_rotation = number / 100;
        let remainder = number % 100;

        let init = dial;
        ans += total_rotation;

        if prefix == "L" {
            dial -= remainder;
            if init > 0 && dial <= 0 {
                ans += 1;
            }
        } else {
            dial += number;
            if dial % 100 < init {
                ans += 1;
            }
        }

        dial %= 100;
        if dial < 0 {
            dial += 100;
        }
    }

    println!("ans={ans}")
}
