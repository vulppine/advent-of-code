use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Coordinates(pub isize, pub isize);

impl Coordinates {
    // move towards another set of coordinates at some
    // acceleration
    fn move_towards(&mut self, other: &Self, acc: usize) {
        let direction = other.get_direction(self);
        self.0 = match direction.0 {
            Ordering::Greater => self.0 + acc as isize,
            Ordering::Less => self.0 - acc as isize,
            Ordering::Equal => self.0,
        };

        self.1 = match direction.1 {
            Ordering::Greater => self.1 + acc as isize,
            Ordering::Less => self.1 - acc as isize,
            Ordering::Equal => self.1,
        };
    }

    fn delta_between(&self, other: &Self) -> (isize, isize) {
        ((other.0 - self.0).abs(), (other.1 - self.1).abs())
    }

    // gets if a coordinate is not level with another coordinate
    fn is_diagonal(&self, other: &Self) -> bool {
        let delta = self.delta_between(other);

        delta.0 != 0 && delta.1 != 0
    }

    fn is_level(&self, other: &Self) -> bool {
        !self.is_diagonal(other)
    }

    // get the direction from self to other (in terms of order/order)
    fn get_direction(&self, other: &Self) -> (Ordering, Ordering) {
        (self.0.cmp(&other.0), self.1.cmp(&other.1))
    }
}

#[derive(Debug, Hash)]
struct Vector {
    head: Coordinates,
    tail: Coordinates,
    pos: Option<Coordinates>, // for iterating from head->tail
    acc: usize,
}

impl Vector {
    fn new(head: Coordinates, tail: Coordinates, acc: usize) -> Self {
        Vector {
            head,
            tail,
            pos: None,
            acc,
        }
    }

    fn is_level(&self) -> bool {
        self.head.is_level(&self.tail)
    }

    fn is_45deg(&self) -> bool {
        let delta = self.head.delta_between(&self.tail);

        delta.0 == delta.1 && delta.0 != 0 && delta.1 != 0
    }

    fn from_string(string: &str) -> Self {
        let coord_set = string
            .split(" -> ")
            .map(|s| s.split(','))
            .map(|v| {
                v.fold(Vec::new(), |mut s, e| {
                    s.push(e.parse::<isize>().unwrap());
                    s
                })
            })
            .map(|c| Coordinates(c[0], c[1]))
            .collect::<Vec<Coordinates>>();

        Self::new(coord_set[0].clone(), coord_set[1].clone(), 1)
    }
}

impl Iterator for Vector {
    type Item = Coordinates;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.is_none() {
            self.pos = Some(self.tail.clone());
            return self.pos.clone();
        }

        if self.pos.as_ref().unwrap().eq(&self.head) {
            self.pos = None;
            return None;
        }

        if self.head.is_diagonal(&self.tail) && !self.is_45deg() {
            return None;
        }

        self.pos
            .as_mut()
            .unwrap()
            .move_towards(&self.head, self.acc);

        self.pos.clone()
    }
}

#[derive(Debug)]
struct VectorMap {
    pub map: HashMap<Coordinates, usize>,
    vectors: Vec<Vector>,
}

impl VectorMap {
    fn new() -> Self {
        VectorMap {
            map: HashMap::new(),
            vectors: Vec::new(),
        }
    }

    fn from_file(file: File) -> Self {
        let lines = BufReader::new(file).lines();
        let mut vmap = VectorMap::new();

        for l in lines.flatten() {
            let v = Vector::from_string(&l);

            if v.is_level() || v.is_45deg() {
                vmap.add_vector(v);
            }
        }

        vmap
    }

    fn add_vector(&mut self, vec: Vector) {
        self.vectors.push(vec);
    }

    fn populate_map(&mut self) {
        self.map.clear();
        for v in self.vectors.iter_mut() {
            for c in v {
                if let Some(m) = self.map.get(&c) {
                    let n = m + 1;
                    self.map.insert(c, n);
                } else {
                    self.map.insert(c, 1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn vector_45deg_test() {
        let vec = Vector::from_string("0,0 -> 5,5");
        println!("{}", vec.is_45deg());
        for c in vec {
            println!("{:?}", c);
        }

        let wrong = Vector::from_string("0,0 -> 5,2");
        println!("{}", wrong.is_45deg());
    }

    #[test]
    fn test_day_five() {
        let file = File::open("res/day_five.input").unwrap();
        let mut vmap = VectorMap::from_file(file);

        vmap.populate_map();

        let count = vmap.map.iter().filter(|v| v.1 > &1).count();

        println!("{}", count);
    }

    #[test]
    fn vmap_parse_test() {
        let file = File::open("res/day_five.test").unwrap();
        let mut vmap = VectorMap::from_file(file);

        vmap.populate_map();

        println!("{:?}", vmap);

        vmap.map
            .iter()
            .filter(|v| v.1 > &1)
            .for_each(|v| println!("{:?}", v));
    }

    #[test]
    fn vector_parse_test() {
        let vec = Vector::from_string("1,2 -> 3,4");

        println!("{:?}", vec);
    }

    #[test]
    fn test_vector_map() {
        let mut vmap = VectorMap::new();
        vmap.add_vector(test_vector());
        vmap.populate_map();
        println!("{:?}", vmap);

        vmap.add_vector(test_vector_ew());
        vmap.populate_map();
        println!("{:?}", vmap);
    }

    fn test_vector() -> Vector {
        println!("creating vector");
        let coord_b = Coordinates(0, 0);
        let coord_a = Coordinates(0, 5);

        Vector::new(coord_a, coord_b, 1)
    }

    fn test_vector_ew() -> Vector {
        println!("creating vector");
        let coord_b = Coordinates(0, 5);
        let coord_a = Coordinates(0, 0);

        Vector::new(coord_a, coord_b, 1)
    }

    #[test]
    fn vector_move() {
        let mut test_vec = test_vector();
        let test_vec_ew = test_vector_ew();

        for c in test_vec {
            println!("{:?}", c);
        }

        for c in test_vec_ew {
            println!("{:?}", c);
        }
    }
}
