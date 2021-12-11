use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
struct LanternFish {
    timer: usize,
    spawning: bool,
}

impl LanternFish {
    fn new(timer: usize) -> Self {
        LanternFish {
            timer,
            spawning: false,
        }
    }

    fn dec(&mut self) {
        self.timer -= 1;
        if self.timer == 0 {
            self.spawning = true;
        }
    }

    fn set(&mut self, n: usize) {
        self.timer = n;
        if self.timer == 0 {
            self.spawning = true;
        } else {
            self.spawning = false;
        }
    }
}

struct FishSimulation {
    fish: Vec<LanternFish>,
}

impl FishSimulation {
    fn new(fish: Vec<LanternFish>) -> Self {
        FishSimulation { fish }
    }

    fn new_from_file(file: File) -> Self {
        let fish = file
            .bytes()
            .flatten()
            .map(|b| (b as char).to_digit(10))
            .filter(|b| b.is_some())
            .flatten()
            .map(|b| LanternFish::new(b as usize))
            .collect::<Vec<LanternFish>>();

        Self::new(fish)
    }

    fn process(&mut self, days: usize) {
        // store fish indexes here
        let mut spawning_fish: usize = 0;

        for _ in 0..days {
            // println!("{:?}", self.fish.len());
            /*
            for f in self.fish.iter_mut().enumerate() {
                if f.1.timer == 0 && f.1.spawning {
                    spawning_fish.push(f.0);
                    f.1.timer = 6;
                } else {
                    f.1.dec();
                }
            }
            */

            spawning_fish = Self::dec_recur(&mut self.fish);

            for f in 0..spawning_fish {
                self.fish.push(LanternFish::new(8));
            }

            spawning_fish = 0;
        }
    }

    fn dec_recur(fish: &mut [LanternFish]) -> usize {
        match fish.len() {
            0 => 0,
            1 => {
                if fish[0].timer == 0 {
                    fish[0].timer = 6;
                    1
                } else {
                    fish[0].dec();
                    0
                }
            }
            _ => {
                let fish_len = fish.len();
                Self::dec_recur(&mut fish[0..fish_len / 2])
                    + Self::dec_recur(&mut fish[fish_len / 2..fish_len])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_day_six() {
        let file = File::open("res/day_six.input").unwrap();
        let mut sim = FishSimulation::new_from_file(file);

        sim.process(80);
        println!("{}", sim.fish.len());
    }

    #[test]
    fn dec_recur() {
        let mut fish = vec![
            LanternFish::new(3),
            LanternFish::new(4),
            LanternFish::new(3),
            LanternFish::new(1),
            LanternFish::new(2),
        ];

        FishSimulation::dec_recur(&mut fish);
        println!("{:?}", fish);
    }

    #[test]
    fn fish_test() {
        let mut sim = FishSimulation::new(vec![
            LanternFish::new(3),
            LanternFish::new(4),
            LanternFish::new(3),
            LanternFish::new(1),
            LanternFish::new(2),
        ]);

        sim.process(256);

        println!("{}", sim.fish.len());
    }
}
