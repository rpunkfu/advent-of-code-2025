use std::collections::BTreeMap;
use std::io::BufRead;

const IS_TEST: bool = false;

#[derive(Debug, Default, PartialEq, Eq)]
enum State {
    #[default]
    AggregateRange,
    ValidateIdentification,
}

#[derive(Debug, Default)]
struct RangeSet {
    ranges: BTreeMap<u64, u64>,
}

impl RangeSet {
    fn insert(&mut self, start: u64, end: u64) {
        let mut start = start;
        let mut end = end;

        if let Some((&old_start, &old_end)) = self.ranges.range(..=start).next_back()
            && old_end >= start - 1
        {
            start = start.min(old_start);
            end = end.max(old_end);

            self.ranges.remove(&old_start);
        }

        while let Some((&old_start, &old_end)) = self.ranges.range(start..=end).next() {
            if old_start < start - 1 && old_end > end + 1 {
                continue;
            }

            start = start.min(old_start);
            end = end.max(old_end);

            self.ranges.remove(&old_start);
            break;
        }

        if let Some((&old_start, &old_end)) = self.ranges.range(end..).next()
            && old_start <= end + 1
        {
            start = start.min(old_start);
            end = end.max(old_end);

            self.ranges.remove(&old_start);
        }

        self.ranges.insert(start, end);
    }

    fn contains(&self, value: u64) -> bool {
        (self.ranges.range(..=value).next_back())
            .is_some_and(|(&start, &end)| start <= value && value <= end)
    }

    fn total_len(&self) -> u64 {
        dbg!(&self.ranges);
        self.ranges.iter().map(|(start, end)| end - start + 1).sum()
    }
}

fn main() {
    let file_path = std::path::Path::new(match IS_TEST {
        true => ".fixtures/test.txt",
        false => ".fixtures/input.txt",
    });

    let mut solution_1: u64 = 0;
    let mut solution_2: u64 = 0;

    let mut range_set: RangeSet = Default::default();
    let mut state: State = Default::default();

    let file = std::fs::File::open(file_path).expect("file not found");
    let reader = std::io::BufReader::new(file);
    for line_buffer in reader.lines().map_while(Result::ok) {
        if line_buffer.is_empty() {
            solution_2 = range_set.total_len();
            state = State::ValidateIdentification;
            continue;
        }

        if matches!(state, State::AggregateRange) {
            let (start, end) = (line_buffer.split_once('-'))
                .and_then(|(start, end)| (start.parse().ok()?, end.parse().ok()?).into())
                .expect("invalid range");

            range_set.insert(start, end);
        }

        if matches!(state, State::ValidateIdentification) {
            let id = line_buffer.trim().parse().expect("invalid id");
            if range_set.contains(id) {
                solution_1 += 1;
            }
        }
    }

    println!("Solution 1: {}", solution_1);
    println!("Solution 2: {}", solution_2);
}
