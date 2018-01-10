use day_19::Direction::*;

const DAY_19_INPUT: &'static str = include_str!("../data/day_19_real_input");

pub fn run() -> Result<(), &'static str> {
    println!("*** Day 19: A Series of Tubes ***");
    println!("Input: {}", DAY_19_INPUT);
    println!("solution 1: {:?}", solution_1(DAY_19_INPUT)?);
    println!("solution 2: {:?}", total_steps_to_end(DAY_19_INPUT)?);
    Ok(())
}

fn solution_1(s: &str) -> Result<String, &'static str> {
    let m = Maze::from_str(s)?;
    let runner = MazeRunner::new(&m)?;
    let steps: String = runner
        .filter_map(|cursor| match m.char_at(cursor.coord) {
            Some(c) if c.is_alphabetic() => Some(c),
            _ => None,
        })
        .collect();
    Ok(steps)
}

fn total_steps_to_end(s: &str) -> Result<usize, &'static str> {
    let m = Maze::from_str(s)?;
    let runner = MazeRunner::new(&m)?;
    let steps_needed = runner.count() + 1; // need to count the first non-step
    Ok(steps_needed)
}

#[derive(Debug)]
struct Maze {
    _data: Vec<char>,
    rows: usize,
    cols: usize,
    max_i: usize,
    max_j: usize,
}

impl Maze {
    fn from_str(s: &str) -> Result<Maze, &'static str> {
        let data: Vec<_> = s.split("\n")
            .map(|row| {
                let chars: Vec<_> = row.chars().collect();
                chars
            })
            .collect();
        Maze::new(&data)
    }

    fn new(data: &Vec<Vec<char>>) -> Result<Maze, &'static str> {
        let rows = data.len();
        let cols = data.iter().map(|row| row.len()).next().unwrap_or(0);
        if data.iter().map(|row| row.len()).any(|l| l != cols) {
            Err("Not all rows have the same number of columns :(")
        } else {
            let max_i = rows - 1;
            let max_j = cols - 1;
            let _data = data.iter()
                .fold(Vec::with_capacity(rows * cols), |acc, row| {
                    row.iter().fold(acc, |mut inner_acc, c| {
                        inner_acc.push(*c);
                        inner_acc
                    })
                });
            Ok(Maze {
                _data,
                rows,
                cols,
                max_i,
                max_j,
            })
        }
    }

    fn get_row(&self, i: usize) -> Option<&[char]> {
        if i < self.max_i {
            Some(&self._data[i * self.cols..i * self.cols + self.cols])
        } else {
            None
        }
    }

    fn char_at(&self, Coord { i, j }: Coord) -> Option<char> {
        if i <= self.max_i && j <= self.max_j {
            let idx = self.cols * i + j;
            self._data.get(idx).map(|c| *c)
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_options(&self) -> [Direction; 2] {
        match self {
            &Up | &Down => [Left, Right],
            &Left | &Right => [Up, Down],
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Coord {
    i: usize,
    j: usize,
}

struct TurnNavigator<'a> {
    maze: &'a Maze,
    pivot: Coord,
    search_directions: [Direction; 2],
    dir_1_coord: Option<Coord>,
    dir_2_coord: Option<Coord>,
}

impl<'a> TurnNavigator<'a> {
    fn new(maze: &'a Maze, pivot: Coord, search_directions: [Direction; 2]) -> TurnNavigator<'a> {
        let dir_1_coord = next_coord(maze, pivot, search_directions[0]);
        let dir_2_coord = next_coord(maze, pivot, search_directions[1]);
        TurnNavigator {
            maze,
            pivot,
            search_directions,
            dir_1_coord,
            dir_2_coord,
        }
    }

    fn find_exit(&mut self) -> Option<Cursor> {
        let mut found_next: Option<Cursor> = None;
        while found_next.is_none() && (self.dir_1_coord.is_some() || self.dir_2_coord.is_some()) {
            let dir_1_char = self.dir_1_coord.and_then(|coord| self.maze.char_at(coord));
            let dir_2_char = self.dir_2_coord.and_then(|coord| self.maze.char_at(coord));

            fn work(
                pivot: Coord,
                dir_char: Option<char>,
                dir: Direction,
                found_cursor: &mut Option<Cursor>,
                this_dir_coord: &mut Option<Coord>,
                other_dir_coord: &mut Option<Coord>,
            ) {
                fn set_cursor_found(
                    inner_dir: Direction,
                    pivot: Coord,
                    inner_found_cursor: &mut Option<Cursor>,
                    inner_this_dir_coord: &mut Option<Coord>,
                    inner_other_dir_coord: &mut Option<Coord>,
                ) {
                    *inner_found_cursor = Some(Cursor {
                        coord: pivot,
                        direction: inner_dir,
                    });
                    *inner_this_dir_coord = None;
                    *inner_other_dir_coord = None;
                };
                match dir_char {
                    Some(c) if (c == '-' || c.is_alphabetic()) && (dir == Left || dir == Right) => {
                        set_cursor_found(dir, pivot, found_cursor, this_dir_coord, other_dir_coord)
                    }
                    Some(c) if (c == '|' || c.is_alphabetic()) && dir == Up || dir == Down => {
                        set_cursor_found(dir, pivot, found_cursor, this_dir_coord, other_dir_coord)
                    }
                    Some('+') => {
                        set_cursor_found(dir, pivot, found_cursor, this_dir_coord, other_dir_coord);
                    }
                    Some(' ') | None => *this_dir_coord = None, // stop looking in this direction
                    _ => (), // weird, but let's keep looing for now
                }
            };
            work(
                self.pivot,
                dir_1_char,
                self.search_directions[0],
                &mut found_next,
                &mut self.dir_1_coord,
                &mut self.dir_2_coord,
            );
            if found_next.is_none() {
                work(
                    self.pivot,
                    dir_2_char,
                    self.search_directions[1],
                    &mut found_next,
                    &mut self.dir_2_coord,
                    &mut self.dir_1_coord,
                );
            }

            if let Some(dir_1_coord) = self.dir_1_coord {
                self.dir_1_coord = next_coord(self.maze, dir_1_coord, self.search_directions[0]);
            }
            if let Some(dir_2_coord) = self.dir_2_coord {
                self.dir_2_coord = next_coord(self.maze, dir_2_coord, self.search_directions[1]);
            }
        }
        found_next
    }
}

fn next_coord(maze: &Maze, current: Coord, direction: Direction) -> Option<Coord> {
    match direction {
        Up if current.i > 0 => Some(Coord {
            i: current.i - 1,
            ..current
        }),
        Down if current.i < maze.rows - 1 => Some(Coord {
            i: current.i + 1,
            ..current
        }),
        Left if current.j > 0 => Some(Coord {
            j: current.j - 1,
            ..current
        }),
        Right if current.j < maze.cols - 1 => Some(Coord {
            j: current.j + 1,
            ..current
        }),
        _ => None,
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Cursor {
    coord: Coord,
    direction: Direction,
}

#[derive(Debug)]
struct MazeRunner<'a> {
    maze: &'a Maze,
    cursor: Option<Cursor>,
}

impl<'a> MazeRunner<'a> {
    fn new(maze: &'a Maze) -> Result<MazeRunner<'a>, &'static str> {
        let cursor = Some(Cursor {
            coord: find_entry(maze)?,
            direction: Direction::Down,
        });
        Ok(MazeRunner { maze, cursor })
    }

    fn next_cursor(&self) -> Option<Cursor> {
        self.cursor.and_then(|current| {
            next_coord(self.maze, current.coord, current.direction).and_then(|next_coord| {
                match self.maze.char_at(next_coord) {
                    Some(w) if w.is_whitespace() => None,
                    None => None,
                    Some('+') => {
                        let mut turn_nav = TurnNavigator::new(
                            self.maze,
                            next_coord,
                            current.direction.turn_options(),
                        );
                        turn_nav.find_exit()
                    }
                    Some(_) => Some(Cursor {
                        coord: next_coord,
                        direction: current.direction,
                    }),
                }
            })
        })
    }
}

impl<'a> Iterator for MazeRunner<'a> {
    type Item = Cursor;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next_cursor();
        self.cursor = next;
        next
    }
}

fn find_entry(maze: &Maze) -> Result<Coord, &'static str> {
    match maze.get_row(0) {
        Some(row_1) => match row_1.iter().enumerate().find(|&(_, c)| c == &'|') {
            Some((idx, _)) => Ok(Coord { i: 0, j: idx }),
            None => Err("Could not find entrance char ('|') in the first row."),
        },
        None => Err("Could not find the first row."),
    }
}

#[cfg(test)]
mod tests {

    use day_19::*;

    const TEST_INPUT: &'static str = include_str!("../data/day_19_test_input");

    #[test]
    fn from_test_input_str_test() {
        assert!(Maze::from_str(TEST_INPUT).is_ok());
    }

    #[test]
    fn get_coord_test() {
        let m = Maze::from_str(TEST_INPUT).unwrap();
        assert_eq!(m.char_at(Coord { i: 3, j: 1 }), Some('F'));
    }

    #[test]
    fn from_real_input_str_test() {
        assert!(Maze::from_str(DAY_19_INPUT).is_ok());
    }

    #[test]
    fn maze_runner_sol_1_test() {
        let steps = solution_1(TEST_INPUT).unwrap();
        assert_eq!(steps.as_str(), "ABCDEF");
    }

    #[test]
    fn maze_runner_real_sol_1_test() {
        let steps = solution_1(DAY_19_INPUT).unwrap();
        assert_eq!(steps.as_str(), "QPRYCIOLU");
    }

    #[test]
    fn maze_runner_total_steps_to_end_test() {
        let steps = total_steps_to_end(TEST_INPUT).unwrap();
        assert_eq!(steps, 38);
    }

}
