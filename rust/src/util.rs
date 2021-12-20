// Common utility functions and structs.
//
use std::convert::TryInto;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// # Coordinate
///
/// Coordinates on a cardinal plane.
pub type Coordinate = (isize, isize);

#[derive(Clone)]
pub enum PlaneAxis {
    X,
    Y,
}

impl PlaneAxis {
    pub fn reflect_coordinate(&self, c: Coordinate, l: isize) -> Coordinate {
        match self {
            PlaneAxis::X => ((2 * l) - c.0, c.1),
            PlaneAxis::Y => (c.0, (2 * l) - c.1),
        }
    }

    pub fn from_char(c: char) -> Self {
        match c {
            'x' => PlaneAxis::X,
            'y' => PlaneAxis::Y,
            _ => {
                panic!("unsupported");
            }
        }
    }
}

/// # Direction
///
/// Directions, in cardinal format.
pub enum Direction {
    North,
    NorthWest,
    NorthEast,
    South,
    SouthWest,
    SouthEast,
    West,
    East,
}

impl Direction {
    pub fn to_coords(&self) -> Coordinate {
        match self {
            Direction::North => (0, -1),
            Direction::NorthWest => (-1, -1),
            Direction::NorthEast => (1, -1),
            Direction::South => (0, 1),
            Direction::SouthWest => (-1, 1),
            Direction::SouthEast => (1, 1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
        }
    }

    pub fn move_coords(&self, coords: Coordinate) -> Coordinate {
        let dir = self.to_coords();

        (coords.0 + dir.0, coords.1 + dir.1)
    }
}

/// # Table
///
/// Table, indexable by X and Y.
#[derive(Debug)]
pub struct Table<T> {
    pub rows: Vec<Vec<T>>,
    pub row_size: Option<usize>,
}

impl<T: fmt::Display> fmt::Display for Table<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in &self.rows {
            for e in r {
                write!(f, "{}", e).unwrap();
            }
            writeln!(f).unwrap();
        }

        Ok(())
    }
}

impl<T> Table<T> {
    pub fn new() -> Self {
        Table {
            rows: Vec::new(),
            row_size: None,
        }
    }

    fn is_invalid_coord(&self, x: isize, y: isize) -> bool {
        (x < 0 || x > (self.row_size.unwrap() - 1).try_into().unwrap())
            || (y < 0 || y > (self.rows.len() - 1).try_into().unwrap())
    }

    pub fn get_elem_mut_at_coord(&mut self, c: Coordinate) -> Option<&mut T> {
        self.get_elem_mut_at(c.0, c.1)
    }

    pub fn get_elem_mut_at(&mut self, x: isize, y: isize) -> Option<&mut T> {
        if self.is_invalid_coord(x, y) {
            return None;
        }

        if let Some(r) = self.rows.get_mut(y as usize) {
            r.get_mut(x as usize)
        } else {
            None
        }
    }

    pub fn get_elem_at_coord(&self, c: Coordinate) -> Option<&T> {
        self.get_elem_at(c.0, c.1)
    }

    // lazy, so we'll just throw isizes into this
    pub fn get_elem_at(&self, x: isize, y: isize) -> Option<&T> {
        if self.is_invalid_coord(x, y) {
            return None;
        }

        if let Some(r) = self.rows.get(y as usize) {
            r.get(x as usize)
        } else {
            None
        }
    }

    pub fn get_elems_around(&self, x: isize, y: isize) -> Vec<Option<(&T, Coordinate)>> {
        /*
        // let up = (x, y + 1);
        let down = (x, y - 1);
        let left = (x - 1, y);
        let right = (x + 1, y);
        */
        let n = Direction::North.move_coords((x, y));
        let s = Direction::South.move_coords((x, y));
        let w = Direction::West.move_coords((x, y));
        let e = Direction::East.move_coords((x, y));
        let nw = Direction::NorthWest.move_coords((x, y));
        let ne = Direction::NorthEast.move_coords((x, y));
        let sw = Direction::SouthWest.move_coords((x, y));
        let se = Direction::SouthEast.move_coords((x, y));

        vec![
            self.get_elem_at_coord(n).map(|r| (r, n)),
            self.get_elem_at_coord(s).map(|r| (r, s)),
            self.get_elem_at_coord(w).map(|r| (r, w)),
            self.get_elem_at_coord(e).map(|r| (r, e)),
            self.get_elem_at_coord(nw).map(|r| (r, nw)),
            self.get_elem_at_coord(ne).map(|r| (r, ne)),
            self.get_elem_at_coord(sw).map(|r| (r, sw)),
            self.get_elem_at_coord(se).map(|r| (r, se)),
        ]
    }

    pub fn insert_row(&mut self, row: Vec<T>) {
        if let Some(s) = self.row_size {
            if row.len() != s {
                panic!("inequal row size");
            }
        } else {
            self.row_size = Some(row.len());
        }

        self.rows.push(row);
    }
}

impl Table<u32> {
    pub fn from_file(file: File) -> Self {
        let mut table = Self::new();
        let buf_file = BufReader::new(file);

        buf_file
            .lines()
            .map(|l| {
                l.unwrap()
                    .bytes()
                    .map(|b| (b as char).to_digit(10))
                    .filter(|d| d.is_some())
                    .flatten()
                    .collect::<Vec<u32>>()
            })
            .for_each(|v| table.insert_row(v));

        table
    }
}

pub fn coords_to_table(coords: &mut [Coordinate], mark: char, empty: char) -> Table<char> {
    let mut t = Table::new();
    // get the max X of our coordinates (this will dictate
    // how many columns we have
    coords.sort_by(|a, b| b.0.cmp(&a.0));
    let max_x = coords[0].0;
    // get the max Y of our coordinates (dictates how many
    // rows we have)
    coords.sort_by(|a, b| b.1.cmp(&a.1));
    let max_y = coords[0].1;

    for _ in 0..=max_y {
        let mut r: Vec<char> = Vec::new();
        r.resize_with(max_x as usize + 1, || empty);
        t.insert_row(r);
    }

    for c in coords {
        *t.get_elem_mut_at_coord(*c).unwrap() = mark;
    }

    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_table() {
        let mut coords = [(0, 0), (5, 5)];

        let table = coords_to_table(&mut coords, '#', '.');

        println!("{}", table);
    }

    #[test]
    fn test_table_grabbing() {
        let mut table = Table::new();
        let rows = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        for r in rows {
            table.insert_row(r);
        }

        println!("{:?}", table.get_elems_around(1, 1));
    }
}
