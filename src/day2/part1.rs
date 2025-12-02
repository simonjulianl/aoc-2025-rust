use std::fs;

fn find_invalid_ids(input: &str) -> Vec<u64> {
    let ranges: Vec<&str> = input.split(',').collect();
    let mut invalid_ids: Vec<u64> = Vec::new();

    // Helper lambda
    let is_invalid_id = |s: &str| -> bool {
        let len = s.len();
        if len > 0 && len % 2 == 0 {
            let mid = len / 2;
            let first_half = &s[..mid];
            let second_half = &s[mid..];
            return first_half == second_half;
        }

        false
    };

    for range_str in ranges {
        let parts: Vec<&str> = range_str.split('-').collect();
        let start = parts[0].parse::<u64>().unwrap();
        let end = parts[1].parse::<u64>().unwrap();

        for id in start..=end {
            let id_str = id.to_string();
            if is_invalid_id(&id_str) {
                invalid_ids.push(id);
            }
        }
    }

    invalid_ids
}

fn main() {
    let file_path = "input.txt";

    // Attempt to read the file contents
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let invalid_ids: u64 = find_invalid_ids(&contents).iter().sum();

    println!("{invalid_ids}");
}
