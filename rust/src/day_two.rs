#[derive(Debug)]
struct Position {
    horizontal: isize,
    depth: isize,
    aim: isize,
}

enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl Command {
    pub fn parse(command: String) -> Result<Self, String> {
        let action = command.split(' ').collect::<Vec<&str>>();

        if action.len() != 2 {
            return Err("too long".to_string());
        }

        let units: usize = action[1].parse().map_err(|_| "parse error".to_string())?;

        match action[0] {
            "forward" => Ok(Command::Forward(units)),
            "down" => Ok(Command::Down(units)),
            "up" => Ok(Command::Up(units)),
            _ => Err("incorrect command".to_string()),
        }
    }
}

impl Position {
    pub fn new() -> Self {
        Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    pub fn change_pos(mut self, command: &Command) -> Self {
        match command {
            Command::Forward(delta) => {
                self.horizontal += *delta as isize;
            }

            Command::Down(delta) => {
                self.depth += *delta as isize;
            }

            Command::Up(delta) => {
                self.depth -= *delta as isize;
            }
        }

        self
    }

    pub fn change_pos_aim(mut self, command: &Command) -> Self {
        match command {
            Command::Forward(delta) => {
                self.horizontal += *delta as isize;
                self.depth += self.aim * *delta as isize;
            }

            Command::Down(delta) => {
                self.aim += *delta as isize;
            }

            Command::Up(delta) => {
                self.aim -= *delta as isize;
            }
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_day_two() {
        let mut test_file = File::open("res/day_two.input").unwrap();
        let mut test_string: String = String::new();

        test_file.read_to_string(&mut test_string).unwrap();

        let cmd_vec: Vec<Command> = test_string
            .lines()
            .map(|i| Command::parse(i.to_string()).unwrap())
            .collect();

        let mut pos = Position::new();
        let mut pos_aim = Position::new();

        for cmd in cmd_vec {
            pos = pos.change_pos(&cmd);
            pos_aim = pos_aim.change_pos_aim(&cmd);
        }

        println!("{}", pos.horizontal * pos.depth);
        println!("{}", pos_aim.horizontal * pos_aim.depth);
    }
}
