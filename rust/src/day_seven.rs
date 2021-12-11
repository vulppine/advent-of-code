use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

struct CrabSubmarines {
    crab_pos: Vec<usize>,
}

impl CrabSubmarines {
    fn new(crab_pos: Vec<usize>) -> Self {
        CrabSubmarines { crab_pos }
    }

    fn new_from_file(file: &mut File) -> Self {
        let mut res: String = String::new();
        file.read_to_string(&mut res);

        let crab_pos = res
            .trim()
            .split(',')
            .map(|b| b.parse::<usize>())
            .filter(|b| b.is_ok())
            .flatten()
            .collect::<Vec<usize>>();

        Self::new(crab_pos)
    }

    fn get_all_possible_vals(&mut self) -> (usize, usize) {
        self.crab_pos.sort_unstable();
        let mut v = (0..=self.crab_pos[self.crab_pos.len() - 1] * 2)
            .map(|v| {
                (
                    v,
                    self.crab_pos.iter().fold(0, |acc, x| match x.cmp(&v) {
                        Ordering::Greater => acc + ((x - v) * ((x - v) + 1) / 2),
                        Ordering::Less => acc + ((v - x) * ((v - x) + 1) / 2),
                        Ordering::Equal => acc,
                    }),
                )
            })
            .collect::<Vec<(usize, usize)>>();

        v.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        v[v.len() - 1]
    }

    fn get_efficient_crabs(&mut self) -> usize {
        let mut crab_map: HashMap<usize, usize> = HashMap::new();
        for c in &self.crab_pos {
            match crab_map.get(c) {
                Some(p) => {
                    let new_p = p + 1;
                    crab_map.insert(*c, new_p);
                }
                None => {
                    crab_map.insert(*c, 1);
                }
            };
        }

        let mut v = crab_map.into_iter().collect::<Vec<(usize, usize)>>();

        v.sort_by(|a, b| a.1.cmp(&b.1));
        // println!("{:?}", v);

        let efficient_val = v[v.len() - 1].0;
        self.crab_pos
            .iter()
            .fold(0, |acc, x| match x.cmp(&efficient_val) {
                Ordering::Greater => acc + (x - efficient_val),
                Ordering::Less => acc + (efficient_val - x),
                Ordering::Equal => acc,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_crabs() {
        let mut crabs = CrabSubmarines::new(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
        println!("{}", crabs.get_efficient_crabs());
        println!("{:?}", crabs.get_all_possible_vals());
    }

    #[test]
    fn test_day_seven() {
        let mut file = File::open("res/day_seven.input").unwrap();

        let mut crabs = CrabSubmarines::new_from_file(&mut file);
        println!("{:?}", crabs.crab_pos);

        crabs.get_all_possible_vals();
        // println!("{}", crabs.get_efficient_crabs());
    }
}
