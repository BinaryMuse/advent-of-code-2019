use std::collections::HashSet;

use crate::{
    computer::Computer,
    grid::{Cell, Direction, Grid, Turn},
};

pub(crate) fn run(input: String) {
    let mut computer = Computer::new(input.clone().into());
    computer.set_yield_on_output(false);
    let mut painted: HashSet<Cell> = HashSet::new();
    let mut visited: HashSet<Cell> = HashSet::new();
    let mut robot = Robot::new();
    start_robot(&mut robot, &mut computer, &mut painted, &mut visited);
    let count = visited.len();
    println!("Part 1: {}", count);

    // Start on a white panel for part 2
    let mut computer = Computer::new(input.clone().into());
    computer.set_yield_on_output(false);
    let mut painted: HashSet<Cell> = HashSet::new();
    painted.insert((0, 0).into());
    let mut visited: HashSet<Cell> = HashSet::new();
    let mut robot = Robot::new();
    start_robot(&mut robot, &mut computer, &mut painted, &mut visited);

    let mut grid: Grid<bool> = Grid::new();
    for cell in painted.iter() {
        grid.insert(cell, true);
    }

    println!("Part 2 ({}):", painted.len());

    for row in grid.rows_iter() {
        print!("\n");
        for painted in row {
            match painted {
                Some(true) => print!("#"),
                _ => print!(" "),
            }
        }
    }

    println!("\n");
}

fn start_robot(
    robot: &mut Robot,
    computer: &mut Computer,
    painted: &mut HashSet<Cell>,
    visited: &mut HashSet<Cell>,
) {
    while computer.is_halted() == false {
        let current_color = painted.get(&robot.position).map(|_| 1).unwrap_or(0);
        computer.push_input(current_color);
        computer.run();

        let paint_num = computer.next_output().unwrap();
        if paint_num == 1 {
            painted.insert(robot.position);
            visited.insert(robot.position);
        } else {
            painted.remove(&robot.position);
        }
        let turn_num = computer.next_output().unwrap();
        let turn = if turn_num == 1 {
            Turn::Right
        } else {
            Turn::Left
        };
        robot.turn(turn);
        robot.forward();
    }
}

struct Robot {
    position: Cell,
    direction: Direction,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            position: Cell(0, 0),
            direction: Direction::North,
        }
    }

    pub fn turn(&mut self, turn: Turn) {
        self.direction = self.direction.turn(turn);
    }

    pub fn forward(&mut self) {
        let delta: Cell = self.direction.into();
        self.position = self.position + delta;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_thing() {
        assert!(true);
    }
}
