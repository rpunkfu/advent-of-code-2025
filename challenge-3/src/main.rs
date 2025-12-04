use std::io::Read;

const IS_TEST: bool = false;

const BANK_BUFFER: usize = 256;
const BANK_STATE: usize = 12;

struct Bank {
    length: usize,
    buffer: [u8; BANK_BUFFER],
}

impl Bank {
    fn new(length: usize) -> Self {
        Self {
            length,
            buffer: [0; BANK_BUFFER],
        }
    }

    fn reset(&mut self) {
        self.buffer = [0; BANK_BUFFER];
    }

    fn process(&mut self, digit: u8) {
        *((self.buffer.iter_mut())
            .find(|x| **x == 0)
            .expect("buffer full")) = digit;
    }

    fn calculate(&self) -> usize {
        let mut state = [0; BANK_STATE];
        let buffer_used = self.buffer.iter().take_while(|x| **x != 0).count();

        for (idx, digit) in self.buffer.into_iter().enumerate() {
            let buffer_left = buffer_used.saturating_sub(idx + 1);
            let state_start_idx = self.length.saturating_sub(buffer_left + 1);
            if state_start_idx >= self.length {
                break;
            }

            let mut found = false;
            for state_digit in state.iter_mut().take(self.length).skip(state_start_idx) {
                if found {
                    *state_digit = 0;
                    continue;
                }

                if digit > *state_digit {
                    *state_digit = digit;
                    found = true;
                }
            }
        }

        (state.into_iter())
            .take(self.length)
            .fold(0usize, |acc, x| acc * 10 + x as usize)
    }
}

fn main() {
    let file_path = std::path::Path::new(match IS_TEST {
        true => ".fixtures/test.txt",
        false => ".fixtures/input.txt",
    });

    let mut bank_v1 = Bank::new(2);
    let mut bank_v2 = Bank::new(12);

    let mut solution_v1 = 0usize;
    let mut solution_v2 = 0usize;

    let file = std::fs::File::open(file_path).expect("file not found");
    let reader = std::io::BufReader::new(file);

    for character in (reader.bytes()).filter_map(|byte| byte.ok()) {
        if character.is_ascii_control() {
            solution_v1 += bank_v1.calculate();
            solution_v2 += bank_v2.calculate();

            bank_v1.reset();
            bank_v2.reset();

            continue;
        }

        if character.is_ascii_digit() {
            let digit = character - b'0';

            bank_v1.process(digit);
            bank_v2.process(digit);

            continue;
        }
    }

    println!("Solution V1: {}", solution_v1);
    println!("Solution V2: {}", solution_v2);
}
