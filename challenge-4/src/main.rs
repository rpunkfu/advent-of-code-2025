use std::collections::HashSet;
use std::io::BufRead;

const IS_TEST: bool = false;

const NEIGHBORS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

const NEIGHBORS_LIMIT: usize = 4;

type Point = (usize, usize);
type Points = HashSet<Point>;

fn count_paper_neighbors(paper_points: &Points, point: Point) -> usize {
    (NEIGHBORS.iter())
        .filter_map(|(dx, dy)| {
            let nx = point.0.checked_add_signed(*dx)?;
            let ny = point.1.checked_add_signed(*dy)?;
            Some((nx, ny))
        })
        .filter(|pos| paper_points.contains(pos))
        .count()
}

fn validate_forklift_point(paper_points: &Points, point: Point) -> bool {
    count_paper_neighbors(paper_points, point) < NEIGHBORS_LIMIT
}

struct WorkGrid {
    height: usize,
    paper_points: Points,
}

impl WorkGrid {
    fn new() -> Self {
        Self {
            height: 0,
            paper_points: HashSet::new(),
        }
    }

    fn parse_line(&mut self, line_buffer: &[u8]) {
        (line_buffer.iter().enumerate())
            .filter(|(_, c)| **c == b'@')
            .for_each(|(idx, _)| {
                self.paper_points.insert((idx, self.height));
            });

        self.height += 1;
    }

    fn calculate_forklift_points(&self, recursive: bool) -> usize {
        let mut all_paper_points = self.paper_points.clone();
        let mut forklift_point_count = 0;

        loop {
            let mut valid_forklift_point_found = false;
            let paper_points = all_paper_points.clone();

            for &point in paper_points.iter() {
                if validate_forklift_point(&paper_points, point) {
                    valid_forklift_point_found = true;
                    all_paper_points.remove(&point);
                    forklift_point_count += 1;
                }
            }

            if !valid_forklift_point_found || !recursive {
                break;
            }
        }

        forklift_point_count
    }
}

fn main() {
    let file_path = std::path::Path::new(match IS_TEST {
        true => ".fixtures/test.txt",
        false => ".fixtures/input.txt",
    });

    let mut work_grid = WorkGrid::new();

    let file = std::fs::File::open(file_path).expect("file not found");
    let reader = std::io::BufReader::new(file);
    for line_buffer in reader.lines().map_while(Result::ok) {
        work_grid.parse_line(line_buffer.as_bytes());
    }

    let solution_v1 = work_grid.calculate_forklift_points(false);
    let solution_v2 = work_grid.calculate_forklift_points(true);

    println!("Solution V1: {}", solution_v1);
    println!("Solution V2: {}", solution_v2);
}
