use crate::util::*;
use std::collections::HashSet;
use std::fs::File;

#[derive(Debug)]
struct OctopusCave {
    octopi: Table<u32>,
    overcharged_octopi: HashSet<Coordinate>,
    flashed_octopi: HashSet<Coordinate>,
    flash_count: usize,
}

impl OctopusCave {
    pub fn new() -> Self {
        Self {
            octopi: Table::new(),
            overcharged_octopi: HashSet::new(),
            flashed_octopi: HashSet::new(),
            flash_count: 0,
        }
    }

    pub fn from_file(file: File) -> Self {
        Self {
            octopi: Table::<u32>::from_file(file),
            overcharged_octopi: HashSet::new(),
            flashed_octopi: HashSet::new(),
            flash_count: 0,
        }
    }

    pub fn charge_octopi(&mut self) {
        for y in self.octopi.rows.iter_mut().enumerate() {
            for x in y.1.iter_mut().enumerate() {
                *x.1 += 1;

                // since flashing occurs after this step, we have to save a
                // specific set of overcharged octopi just for this
                if *x.1 > 9 {
                    self.overcharged_octopi.insert((x.0 as isize, y.0 as isize));
                }
            }
        }
    }

    pub fn flash_octopus(&mut self, coord: Coordinate) {
        if self.flashed_octopi.contains(&coord) {
            return;
        }

        self.flash_count += 1;
        self.flashed_octopi.insert(coord);

        // love to allocate every time this occurs :despair:
        let coord_set = self
            .octopi
            .get_elems_around(coord.0, coord.1)
            .iter()
            .map(|r| r.map(|c| c.1))
            .flatten()
            .collect::<Vec<Coordinate>>();

        for c in coord_set {
            let e = self.octopi.get_elem_mut_at_coord(c).unwrap();

            *e += 1;

            if *e > 9 {
                self.flash_octopus(c);
            }
        }
    }

    pub fn process_octopi(&mut self) {
        self.charge_octopi();

        // cloning is also ugly
        for o in self.overcharged_octopi.clone() {
            self.flash_octopus(o);
        }

        for o in self.flashed_octopi.clone() {
            *self.octopi.get_elem_mut_at_coord(o).unwrap() = 0;
        }

        self.flashed_octopi.clear();
        self.overcharged_octopi.clear();
    }

    pub fn process_n_times(&mut self, n: usize) {
        for _ in 0..n {
            self.process_octopi();
        }
    }

    pub fn process_until_sync(&mut self) -> usize {
        let mut step = 0;
        while self.flash_count != self.octopi.row_size.unwrap() * self.octopi.rows.len() {
            self.flash_count = 0;
            self.process_octopi();
            step += 1;
        }

        step
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    fn test_octopus_cave() -> OctopusCave {
        let mut octopus_cave = OctopusCave::new();

        let octopi = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 9, 9, 9, 1],
            vec![1, 9, 1, 9, 1],
            vec![1, 9, 9, 9, 1],
            vec![1, 1, 1, 1, 1],
        ];

        for r in octopi {
            octopus_cave.octopi.insert_row(r);
        }

        octopus_cave
    }

    #[test]
    fn test_octopus_input() {
        let mut cave = OctopusCave::from_file(File::open("res/day_eleven.test").unwrap());

        cave.process_n_times(10);

        println!("flash count: {}", cave.flash_count);
    }

    #[test]
    fn test_day_eleven() {
        let mut cave = OctopusCave::from_file(File::open("res/day_eleven.input").unwrap());
        cave.process_n_times(100);
        println!("flash count: {}", cave.flash_count);
        let mut cave = OctopusCave::from_file(File::open("res/day_eleven.input").unwrap());
        let step = cave.process_until_sync();
        println!("flash synced at: {}", step);
    }

    #[test]
    fn test_octopus_processing() {
        let mut cave = test_octopus_cave();
        println!("{:?}", cave);

        cave.process_n_times(2);

        println!("{}", cave.octopi);
    }

    #[test]
    fn test_octopus_charging() {
        let mut octopus_cave = OctopusCave::new();

        let octopi = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 9, 9, 9, 1],
            vec![1, 9, 1, 9, 1],
            vec![1, 9, 9, 9, 1],
            vec![1, 1, 1, 1, 1],
        ];

        for r in octopi {
            octopus_cave.octopi.insert_row(r);
        }

        octopus_cave.charge_octopi();

        println!("{:?}", octopus_cave);
    }
}
