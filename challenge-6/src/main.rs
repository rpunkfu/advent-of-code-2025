use std::io::BufRead;

const IS_TEST: bool = false;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Operation {
    #[default]
    Ignore,
    Multiply,
    Add,
}

impl TryFrom<char> for Operation {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '*' => Ok(Operation::Multiply),
            '+' => Ok(Operation::Add),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Calculation {
    operation: Operation,
    operands: Vec<u64>,
}

impl Calculation {
    fn result(&self) -> u64 {
        match self.operation {
            Operation::Ignore => 0,
            Operation::Multiply => self.operands.iter().product(),
            Operation::Add => self.operands.iter().sum(),
        }
    }

    fn parse_human(lines: &[Vec<char>]) -> Vec<Calculation> {
        let mut calculations: Vec<Calculation> = vec![];

        for line in (lines.iter())
            .map(|line| line.iter().collect::<String>())
            .map(|line| {
                line.split_whitespace()
                    .map(|word| word.to_string())
                    .collect::<Vec<String>>()
            })
        {
            for (idx, token) in line.iter().enumerate() {
                match (token.parse::<u64>(), calculations.get_mut(idx)) {
                    (Ok(operand), Some(calc)) => calc.operands.push(operand),
                    (Ok(operand), None) => calculations.push(Calculation {
                        operation: Operation::Ignore,
                        operands: vec![operand],
                    }),
                    (Err(_), Some(calc)) => {
                        if let Ok(op) = token.chars().next().unwrap_or(' ').try_into() {
                            calc.operation = op;
                        }
                    }
                    (Err(_), None) => {
                        if let Ok(op) = token.chars().next().unwrap_or(' ').try_into() {
                            calculations.push(Calculation {
                                operation: op,
                                operands: vec![],
                            });
                        }
                    }
                }
            }
        }

        calculations
    }

    fn parse_cephalopod(lines: &[Vec<char>]) -> Vec<Calculation> {
        let width = lines.iter().map(|line| line.len()).max().unwrap_or(0) + 1;
        let height = lines.len();

        let transposed: Vec<Vec<char>> = (0..width)
            .map(|col| {
                (0..height)
                    .map(|row| lines[row].get(col).copied().unwrap_or(' '))
                    .collect()
            })
            .collect();

        let mut calculations: Vec<Calculation> = vec![];
        let mut partial = Calculation::default();

        for column in transposed.iter() {
            if column.iter().all(|&c| c == ' ') {
                calculations.push(partial);
                partial = Calculation::default();
                continue;
            }

            if let Some(&op_char) = column.iter().find(|&&c| c == '*' || c == '+') {
                if let Ok(op) = op_char.try_into() {
                    partial.operation = op;
                }
            }

            if let Ok(operand) = (column.iter())
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u64>()
            {
                partial.operands.push(operand);
            }
        }

        calculations
    }
}

fn main() {
    let file_path = std::path::Path::new(match IS_TEST {
        true => ".fixtures/test.txt",
        false => ".fixtures/input.txt",
    });

    let file = std::fs::File::open(file_path).expect("file not found");
    let reader = std::io::BufReader::new(file);

    let lines: Vec<Vec<char>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    let solution_1: u64 = (Calculation::parse_human(&lines).iter())
        .map(|c| c.result())
        .sum();

    let solution_2: u64 = (Calculation::parse_cephalopod(&lines).iter())
        .map(|c| c.result())
        .sum();

    println!("Solution 1: {solution_1}");
    println!("Solution 2: {solution_2}");
}
