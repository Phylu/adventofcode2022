use std::collections::HashMap;

use grid::{grid, Grid};
use log::{debug, trace};

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

const MAX_ROCKS: usize = 2022;
const MAX_ROCKS_2: usize = 1000000000000;
const ROCK_ORDER: [Shape; 5] = [
    Shape::Row,
    Shape::Plus,
    Shape::L,
    Shape::Column,
    Shape::Block,
];
const CAVE_WIDTH: usize = 7;

fn task1(content: &String) -> String {
    let jet_pattern = read_input(content);
    let mut jet_counter: usize = 0;
    let mut cave: Grid<bool> = grid![];

    for i in 0..MAX_ROCKS {
        debug!("Rock {} falling", i);
        let shape = &ROCK_ORDER[i % 5];
        let mut rock = Rock::new(shape, height(&cave));

        // Move Rock to the correct position
        loop {
            let direction = &jet_pattern[jet_counter % jet_pattern.len()];
            if rock.move_possible(&cave, direction) {
                rock = rock.move_in_direction(direction);
            }
            jet_counter += 1;

            if rock.move_possible(&cave, &Direction::Down) {
                rock = rock.move_in_direction(&Direction::Down);
            } else {
                // We can't go down anymore, so we continue to the next rock
                break;
            }
        }

        // Add final rock position to our grid
        cave = place_rock(cave, rock);
    }
    
    draw(&cave);
    height(&cave).to_string()

}

fn task2(content: &String) -> String {
    let mut jumped_height: usize = 0;

    let jet_pattern = read_input(content);
    let mut jet_counter: usize = 0;
    let mut cave: Grid<bool> = grid![];

    // Jet Counter Position, Rock -> Height, #Rocks
    let mut cycle_detector: HashMap<(usize, Rock, Vec<bool>), (usize, usize)> = HashMap::new();

    let mut i: usize = 0;
    while i < MAX_ROCKS_2 {
        debug!("Rock {} falling", i);
        let shape = &ROCK_ORDER[i % 5];
        let mut rock = Rock::new(shape, height(&cave));

        // Cycle Detection!

        // Only start looking for a cycle if we have enough rows for a good state
        if cave.rows() > 10 && jumped_height == 0 {
            // Check for cycle
            let mut cycle_rock = rock.clone();
            for i in 0..cycle_rock.positions.len() {
                cycle_rock.positions[i].0 -= height(&cave) + 3;
            }
            let grid_vec = cave.flatten();
            let top_grid = grid_vec[grid_vec.len() - 10 * CAVE_WIDTH..].to_vec();

            if cycle_detector.contains_key(&(jet_counter % jet_pattern.len(), cycle_rock.clone(), top_grid.clone())) {
                let cycle = cycle_detector.get(&(jet_counter % jet_pattern.len(), cycle_rock.clone(), top_grid.clone())).unwrap();
                let cycle_height = cycle.0;
                let cycle_rocks = i - cycle.1;
                let cycle_length = height(&cave) - cycle_height;
                
                debug!("{} Rocks will produce Height {}", cycle_rocks, cycle_length);

                // Set jumped height & bump i
                let jumpable_rocks = (MAX_ROCKS_2 - i) / cycle_rocks;
                debug!("We are at i: {}. We can jump {} rocks", i, jumpable_rocks);

                jumped_height = jumpable_rocks * cycle_length;
                i += jumpable_rocks * cycle_rocks;

                debug!("i bumped to: {}. We have increased the tower size by {}", i, jumped_height);

                draw(&cave);
                
            } else {
                cycle_detector.insert((jet_counter % jet_pattern.len(), cycle_rock.clone(), top_grid.clone()), (height(&cave), i));
            }
        }


        // Move Rock to the correct position
        loop {
            let direction = &jet_pattern[jet_counter % jet_pattern.len()];
            if rock.move_possible(&cave, direction) {
                rock = rock.move_in_direction(direction);
            }
            jet_counter += 1;

            if rock.move_possible(&cave, &Direction::Down) {
                rock = rock.move_in_direction(&Direction::Down);
            } else {
                // We can't go down anymore, so we continue to the next rock
                break;
            }
        }

        // Add final rock position to our grid
        cave = place_rock(cave, rock);

        i += 1;
    }
    
    draw(&cave);
    (height(&cave) + jumped_height).to_string()


}

#[derive(PartialEq, Debug)]
enum Shape {
    Row,
    Plus,
    L,
    Column,
    Block,
}

#[derive(PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    Down,
}

// Pos(row, column)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(usize, usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rock {
    positions: Vec<Pos>,
}

impl Rock {
    fn new(shape: &Shape, height: usize) -> Rock {
        let mut rock = Rock { positions: vec![] };

        let x: usize = height + 3;
        match shape {
            Shape::Row => {
                rock.positions = vec![Pos(x, 2), Pos(x, 3), Pos(x, 4), Pos(x, 5)];
            },
            Shape::Plus => {
                rock.positions = vec![
                    Pos(x, 3),
                    Pos(x + 1, 2),
                    Pos(x + 1, 3),
                    Pos(x + 1, 4),
                    Pos(x + 2, 3),
                ]
            },
            Shape::L => {
                rock.positions = vec![
                    Pos(x, 2),
                    Pos(x, 3),
                    Pos(x, 4),
                    Pos(x + 1, 4),
                    Pos(x + 2, 4),
                ]
            },
            Shape::Column => {
                rock.positions = vec![Pos(x, 2), Pos(x + 1, 2), Pos(x + 2, 2), Pos(x + 3, 2)]
            },
            Shape::Block => {
                rock.positions = vec![Pos(x, 2), Pos(x, 3), Pos(x + 1, 2), Pos(x + 1, 3)]
            },
        }

        rock
    }

    fn move_possible(&self, cave: &Grid<bool>, direction: &Direction) -> bool {
        for p in self.clone().positions {
            trace!(
                "Checking Position {:?} for Direction {:?} on Grid {:?}",
                p, direction, cave
            );

            match *direction {
                Direction::Left => {
                    if p.1 == 0 || (p.0 < cave.rows() && p.1 > 0 && cave[p.0][p.1 - 1]) {
                        return false;
                    }
                }
                Direction::Right => {
                    if p.1 == CAVE_WIDTH - 1
                        || (p.0 < cave.rows() && p.1 < cave.cols() && cave[p.0][p.1 + 1])
                    {
                        return false;
                    }
                }
                Direction::Down => {
                    if p.0 == 0 || (p.0 <= cave.rows() && cave[p.0 - 1][p.1]) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn move_in_direction(self, direction: &Direction) -> Rock {
        debug!("Move {:?}", direction);
        let mut new_rock = self.clone();
        for i in 0..self.positions.len() {
            match direction {
                Direction::Left => {
                    new_rock.positions[i].1 -= 1;
                }
                Direction::Right => {
                    new_rock.positions[i].1 += 1;
                }
                Direction::Down => {
                    new_rock.positions[i].0 -= 1;
                }
            }
        }

        new_rock
    }
}

fn read_input(content: &String) -> Vec<Direction> {
    let mut jet_pattern: Vec<Direction> = vec![];

    for c in content.chars() {
        match c {
            '<' => jet_pattern.push(Direction::Left),
            '>' => jet_pattern.push(Direction::Right),
            _ => {}
        }
    }

    jet_pattern
}

fn place_rock(mut cave: Grid<bool>, rock: Rock) -> Grid<bool> {
    for p in rock.positions {
        if p.0 >= cave.rows() {
            let additional_rows = p.0 + 1 - cave.rows();
            for _ in 0..additional_rows {
                cave.push_row(vec![false; CAVE_WIDTH]);
            }
        }

        cave[p.0][p.1] = true;
    }

    cave
}

fn height(cave: &Grid<bool>) -> usize {
    for row in (0..cave.rows()).rev() {
        for col in 0..CAVE_WIDTH {
            if cave[row][col] {
                return row + 1;
            }
        }
    }

    0
}

fn draw(cave: &Grid<bool>) {
    for i in (0..cave.rows()).rev() {
        print!("{:04} ", i);
        for j in cave[i].iter() {
            match j {
                true => print!("#"),
                false => print!("."),
            }
        }
        println!();
    }
}

#[cfg(test)]
fn test_input() -> String {
    String::from(
        r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
"#,
    )
}

#[test]
fn test_task1() {
    assert_eq!(task1(&test_input()), "3068");
}

#[test]
fn test_task2() {
    assert_eq!(task2(&test_input()), "1514285714288");
}
