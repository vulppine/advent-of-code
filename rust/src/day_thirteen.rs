use crate::util::{coords_to_table, Coordinate, PlaneAxis};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct TransparentPaper {
    dots: HashSet<Coordinate>,
    fold_ins: Vec<(PlaneAxis, isize)>,
}

impl std::fmt::Display for TransparentPaper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut coords = self.dots.iter().copied().collect::<Vec<Coordinate>>();
        let t = coords_to_table(&mut coords, '#', '.');

        write!(f, "{}", t)
    }
}

impl TransparentPaper {
    fn new() -> Self {
        Self {
            dots: HashSet::new(),
            fold_ins: Vec::new(),
        }
    }

    fn from_file(file: File) -> Self {
        let lines = BufReader::new(file).lines().flatten();

        let mut parse_insts = false;
        let mut dots = HashSet::new();
        let mut fold_ins = Vec::new();
        for l in lines {
            match parse_insts {
                false => {
                    if l.is_empty() {
                        parse_insts = true;
                        continue;
                    }

                    let mut coord_split = l.split(',');
                    let x = coord_split.next().unwrap().parse::<isize>().unwrap();
                    let y = coord_split.next().unwrap().parse::<isize>().unwrap();

                    dots.insert((x, y));
                }
                true => {
                    let mut com_split = l.split('=');
                    let axis = PlaneAxis::from_char(
                        com_split
                            .next()
                            .unwrap()
                            .split(' ')
                            .nth(2)
                            .unwrap()
                            .chars()
                            .next()
                            .unwrap(),
                    );
                    let coord = com_split.next().unwrap().parse::<isize>().unwrap();

                    fold_ins.push((axis, coord));
                }
            }
        }

        Self { dots, fold_ins }
    }

    // filters anything greater than this given axis line
    fn filter_against_axis_line(c: Coordinate, axis: &PlaneAxis, l: isize) -> bool {
        match axis {
            PlaneAxis::X => c.0 > l,
            PlaneAxis::Y => c.1 > l,
        }
    }

    fn fold_across(&mut self, axis: PlaneAxis, l: isize) {
        let dots = self
            .dots
            .iter()
            .filter(|c| Self::filter_against_axis_line(**c, &axis, l))
            .copied()
            .collect::<Vec<Coordinate>>();

        for c in dots {
            self.dots.remove(&c);
            self.dots.insert(axis.reflect_coordinate(c, l));
        }
    }

    fn fold_against_n_instructions(&mut self, n: usize) {
        if n > self.fold_ins.len() {
            panic!("cannot fold that many instructions");
        }

        for i in 0..n {
            let axis = self.fold_ins[i].0.clone();
            let coord = self.fold_ins[i].1;

            self.fold_across(axis, coord);
        }
    }

    fn fold_all(&mut self) {
        self.fold_against_n_instructions(self.fold_ins.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_day_thirteen() {
        let file = File::open("res/day_thirteen.input").unwrap();
        let mut paper = TransparentPaper::from_file(file);

        paper.fold_all();

        println!("{}", paper);
    }

    #[test]
    fn test_from_file() {
        let file = File::open("res/day_thirteen.test").unwrap();
        let mut paper = TransparentPaper::from_file(file);

        paper.fold_against_n_instructions(1);

        println!("{}", paper.dots.len());
    }
}
