use std::io::Read;

const IS_TEST: bool = true;

fn is_invalid_v1(num: u64) -> bool {
    let num_str = num.to_string();
    num_str.len().is_multiple_of(2) && num_str[0..num_str.len() / 2] == num_str[num_str.len() / 2..]
}

fn is_invalid_v2(num: u64) -> bool {
    let num_str = num.to_string();
    let num_bytes = num_str.as_bytes();

    (1..=num_bytes.len() / 2)
        .filter(|&i| num_bytes.len().is_multiple_of(i))
        .any(|i| num_bytes.chunks(i).all(|chunk| chunk == &num_bytes[..i]))
}

fn process_range(start: u64, end: u64) -> (u64, u64) {
    let mut valid_sum_v1 = 0u64;
    let mut valid_sum_v2 = 0u64;

    for num in start..=end {
        if is_invalid_v1(num) {
            valid_sum_v1 += num;
        }

        if is_invalid_v2(num) {
            valid_sum_v2 += num;
        }
    }

    (valid_sum_v1, valid_sum_v2)
}

fn main() {
    let file_path = std::path::Path::new(match IS_TEST {
        true => ".fixtures/test.txt",
        false => ".fixtures/input.txt",
    });

    let mut solution_v1 = 0u64;
    let mut solution_v2 = 0u64;

    let mut curr_range_start: Option<u64> = Some(0);
    let mut curr_range_end: Option<u64> = None;

    let file = std::fs::File::open(file_path).expect("file not found");
    let reader = std::io::BufReader::new(file);

    for character in (reader.bytes()).filter_map(|byte| byte.ok()) {
        // Parse digit of upcoming movement
        if character.is_ascii_digit() {
            match (curr_range_start, curr_range_end) {
                (Some(inner), None) => {
                    curr_range_start = Some(inner * 10 + (character - b'0') as u64);
                }
                (Some(..), Some(inner)) => {
                    curr_range_end = Some(inner * 10 + (character - b'0') as u64);
                }
                (None, None) => unreachable!(),
                (None, Some(..)) => unreachable!(),
            }

            continue;
        }

        // Parse start of range
        if character == b'-' {
            curr_range_end = Some(0);
        }

        // Ranges separator, process range
        if character == b',' {
            match (curr_range_start, curr_range_end) {
                (Some(start), Some(end)) => {
                    let (valid_sum_v1, valid_sum_v2) = process_range(start, end);
                    solution_v1 += valid_sum_v1;
                    solution_v2 += valid_sum_v2;
                }
                (Some(..), None) => println!("Error: Range end not found"),
                (None, Some(..)) => println!("Error: Range start not found"),
                (None, None) => println!("Error: Range start and end not found"),
            }

            curr_range_start = Some(0);
            curr_range_end = None;
        }
    }

    if let Some(start) = curr_range_start
        && let Some(end) = curr_range_end
    {
        let (valid_sum_v1, valid_sum_v2) = process_range(start, end);
        solution_v1 += valid_sum_v1;
        solution_v2 += valid_sum_v2;
    }

    println!("Solution V1: {}", solution_v1);
    println!("Solution V2: {}", solution_v2);
}
