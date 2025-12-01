use std::io::Read;

const DIAL_START: i8 = 50;

fn main() {
    let file_path = std::path::Path::new(".fixtures/input.txt");
    let file = std::fs::File::open(file_path).expect("file not found");
    let reader = std::io::BufReader::new(file);

    let mut solution_1 = 0u32;
    let mut solution_2 = 0u32;

    let mut dial_movement = 0i32;
    let mut dial_movement_multiplier = 1i32;
    let mut dial_position = DIAL_START as i32;

    let mut process_movement = |position: &mut i32, movement: &mut i32, multiplier: &mut i32| {
        if *multiplier == 1 {
            *position += *movement;
            solution_2 += (*position / 100) as u32;

            *position = (*position).rem_euclid(100);
            solution_1 += (*position == 0) as u32;
        }

        if *multiplier == -1 {
            if (*position - *movement) <= 0 {
                let start_bonus = if *position == 0 { 0 } else { 1 };
                solution_2 += ((*position - *movement).abs() / 100 + start_bonus) as u32;
            }

            *position -= *movement;
            *position = (*position).rem_euclid(100);
            solution_1 += (*position == 0) as u32;
        }

        *movement = 0;
    };

    for character in (reader.bytes()).filter_map(|byte| byte.ok()) {
        // New line start -- Process new position, given movement
        if character == b'L' || character == b'R' {
            process_movement(
                &mut dial_position,
                &mut dial_movement,
                &mut dial_movement_multiplier,
            );
        }

        // Parse direction of movement (Left)
        if character == b'L' {
            dial_movement_multiplier = -1;
            continue;
        }

        // Parse direction of movement (Right)
        if character == b'R' {
            dial_movement_multiplier = 1;
            continue;
        }

        // Parse digit of upcoming movement
        if character.is_ascii_digit() {
            dial_movement = dial_movement * 10 + (character - b'0') as i32;
            continue;
        }
    }

    process_movement(
        &mut dial_position,
        &mut dial_movement,
        &mut dial_movement_multiplier,
    );

    println!("Solution 1: {}", solution_1);
    println!("Solution 2: {}", solution_2);
}
