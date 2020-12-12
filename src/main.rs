use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
enum Instruction {
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    Forward(i64),
    Right(i64),
    Left(i64),
}

lazy_static! {
    static ref INSTRUCTION_REGEX: Regex =
        Regex::new(r"^(?P<action>\w)(?P<value>\d+)").expect("invalid regex");
}

fn read_instructions<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in lines {
        let caps = INSTRUCTION_REGEX
            .captures(line)
            .expect("invalid instruction line");
        let action = &caps["action"];
        let value = caps["value"].parse::<i64>().expect("invalid action value");
        let instruction = match action {
            "N" => Instruction::North(value),
            "S" => Instruction::South(value),
            "E" => Instruction::East(value),
            "W" => Instruction::West(value),
            "F" => Instruction::Forward(value),
            "R" => {
                if value % 90 != 0 {
                    panic!("invalid rotation {}", value);
                }
                Instruction::Right(value)
            }
            "L" => {
                if value % 90 != 0 {
                    panic!("invalid rotation {}", value);
                }
                Instruction::Left(value)
            }
            other => panic!("invalid instruction: {}{}", other, value),
        };
        instructions.push(instruction);
    }
    instructions
}

fn rotate_waypoint(x: i64, y: i64, rotation: i64) -> (i64, i64) {
    let r = ((x.pow(2) + y.pow(2)) as f64).sqrt();
    let mut theta = (y as f64).atan2(x as f64);
    theta -= (rotation as f64).to_radians();
    let new_x = (r * theta.cos()).round() as i64;
    let new_y = (r * theta.sin()).round() as i64;
    (new_x, new_y)
}

struct Ship {
    north: i64,
    east: i64,
    waypoint_north: i64,
    waypoint_east: i64,
    facing: (i64, i64),
}

impl Ship {
    fn new() -> Ship {
        Ship {
            north: 0,
            east: 0,
            waypoint_north: 1,
            waypoint_east: 10,
            facing: (1, 0),
        }
    }

    fn plot(&mut self, instructions: &[Instruction]) -> (i64, i64) {
        for instruction in instructions {
            match instruction {
                Instruction::North(value) => self.north += *value,
                Instruction::South(value) => self.north -= *value,
                Instruction::East(value) => self.east += *value,
                Instruction::West(value) => self.east -= *value,
                Instruction::Forward(value) => {
                    let (east, north) = self.facing;
                    self.north += north * value;
                    self.east += east * value;
                }
                Instruction::Right(degrees) => {
                    let (x, y) = self.facing;
                    self.facing = rotate_waypoint(x, y, *degrees);
                }
                Instruction::Left(degrees) => {
                    let (x, y) = self.facing;
                    self.facing = rotate_waypoint(x, y, *degrees * -1);
                }
            }
        }
        (self.north, self.east)
    }

    fn plot_with_waypoint(&mut self, instructions: &[Instruction]) -> (i64, i64) {
        for instruction in instructions {
            match instruction {
                Instruction::North(value) => self.waypoint_north += value,
                Instruction::South(value) => self.waypoint_north -= value,
                Instruction::East(value) => self.waypoint_east += value,
                Instruction::West(value) => self.waypoint_east -= value,
                Instruction::Forward(value) => {
                    self.north += value * self.waypoint_north;
                    self.east += value * self.waypoint_east;
                }
                Instruction::Right(degrees) => {
                    let (new_east, new_north) =
                        rotate_waypoint(self.waypoint_east, self.waypoint_north, *degrees);
                    self.waypoint_east = new_east;
                    self.waypoint_north = new_north;
                }
                Instruction::Left(degrees) => {
                    let (new_east, new_north) =
                        rotate_waypoint(self.waypoint_east, self.waypoint_north, *degrees * -1);
                    self.waypoint_east = new_east;
                    self.waypoint_north = new_north;
                }
            }
        }
        (self.north, self.east)
    }
}

fn main() {
    let instructions = read_instructions(include_str!("../input.txt").lines());
    let (north, east) = Ship::new().plot(&instructions);
    println!("manhattan distance is {}", north.abs() + east.abs());

    let (north, east) = Ship::new().plot_with_waypoint(&instructions);
    println!("manhattan distance is {}", north.abs() + east.abs());
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_JOURNEY: &str = indoc! {"\
    F10
    N3
    F7
    R90
    F11"};

    #[test]
    fn rotation_is_correctly_applied() {
        assert_eq!(rotate_waypoint(1, 0, 90), (0, -1));
        assert_eq!(rotate_waypoint(0, -1, 90), (-1, 0));
        assert_eq!(rotate_waypoint(-1, 0, 90), (0, 1));
        assert_eq!(rotate_waypoint(0, 1, 90), (1, 0));

        assert_eq!(rotate_waypoint(1, 0, -90), (0, 1));
        assert_eq!(rotate_waypoint(0, -1, -90), (1, 0));
        assert_eq!(rotate_waypoint(-1, 0, -90), (0, -1));
        assert_eq!(rotate_waypoint(0, 1, -90), (-1, 0));
    }

    #[test]
    fn plot_is_correctly_calculated() {
        let instructions = read_instructions(TEST_JOURNEY.lines());
        let mut ship = Ship::new();
        let (north, east) = ship.plot(&instructions);
        assert_eq!(25, north.abs() + east.abs());
    }

    #[test]
    fn waypoint_plot_is_correctly_calculated() {
        let instructions = read_instructions(TEST_JOURNEY.lines());
        let mut ship = Ship::new();
        let (north, east) = ship.plot_with_waypoint(&instructions);
        assert_eq!(286, north.abs() + east.abs());
    }
}
