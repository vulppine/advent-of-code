use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Bingo {
    draw_order: Vec<usize>,
    tables: Vec<BingoTable>,
}

impl Bingo {
    fn new() -> Self {
        Bingo {
            draw_order: Vec::new(),
            tables: Vec::new(),
        }
    }

    fn parse_from_file(file: File) -> Self {
        let mut lines = BufReader::new(file).lines().peekable();
        let mut bingo = Self::new();

        if let Some(Ok(line)) = lines.next() {
            bingo.draw_order = line
                .split(',')
                .filter_map(|i| i.parse::<usize>().ok())
                .collect();
        }

        let mut row_set: Vec<Vec<usize>> = Vec::new();
        let mut id = 0;
        for line in lines.flatten() {
            if line.is_empty() {
                if row_set.is_empty() {
                    continue;
                }

                let size = row_set[0].len();
                let mut table = BingoTable::new(size, id);

                table.populate(&row_set);

                bingo.tables.push(table);
                row_set.clear();

                id += 1;

                continue;
            }

            // parse the rows first
            let row: Vec<usize> = line
                .split(' ')
                .filter_map(|i| i.parse::<usize>().ok())
                .collect();

            row_set.push(row);
        }

        if !row_set.is_empty() {
            let size = row_set[0].len();
            let mut table = BingoTable::new(size, id);

            table.populate(&row_set);

            bingo.tables.push(table);
        }

        bingo
    }

    fn process_bingo(&mut self) -> usize {
        for i in &self.draw_order {
            for t in &mut self.tables {
                let coord = t.get_cell_coords(*i);
                if let Some(c) = coord {
                    t.set_cell_bingopos(c, true).unwrap();
                }

                // println!("{}", t);

                if t.has_bingo {
                    return t.calculate_score() * i;
                }
            }
        }

        0
    }

    fn process_least_bingo(&mut self) -> usize {
        let mut to_remove: BTreeSet<usize> = BTreeSet::new();
        for i in &self.draw_order {
            for t in &to_remove {
                if let Ok(id) = self.tables.binary_search_by(|b| b.id.cmp(t)) {
                    self.tables.remove(id);
                }
            }

            let cur_len = self.tables.len();
            for t in self.tables.iter_mut() {
                let coord = t.get_cell_coords(*i);
                if let Some(c) = coord {
                    t.set_cell_bingopos(c, true).unwrap();
                }

                // println!("{}", t);

                if t.has_bingo {
                    if cur_len != 1 {
                        to_remove.insert(t.id);
                    } else {
                        return t.calculate_score() * i;
                    }
                }
            }
        }

        0
    }
}

// nested vec ugly but it works for now
#[derive(Debug, Eq)]
struct BingoTable {
    id: usize,
    rows: Vec<Vec<BingoCell>>,
    size: usize,
    search: HashMap<usize, BingoPos>,
    has_bingo: bool,
}

type BingoPos = (usize, usize);
type BingoCell = (usize, bool);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_coords(&self) -> (i8, i8) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn move_coords(&self, coords: (i8, i8)) -> (i8, i8) {
        let dir = self.to_coords();

        (coords.0 + dir.0, coords.1 + dir.1)
    }
}

impl std::fmt::Display for BingoTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for r in &self.rows {
            write!(f, "[ ").unwrap();
            for c in r {
                write!(f, "{} ", if c.1 { "x" } else { "o" }).unwrap();
            }
            writeln!(f, "]").unwrap();
        }

        Ok(())
    }
}

impl Ord for BingoTable {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for BingoTable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BingoTable {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl BingoTable {
    fn new(size: usize, id: usize) -> BingoTable {
        let mut rows: Vec<Vec<BingoCell>> = Vec::new();

        (0..size).for_each(|_| {
            let mut row: Vec<BingoCell> = Vec::new();
            row.resize_with(size, || (0, false));

            rows.push(row);
        });

        BingoTable {
            id,
            rows,
            size,
            search: HashMap::new(),
            has_bingo: false,
        }
    }

    // yeah
    fn populate(&mut self, rows: &[Vec<usize>]) {
        for i in rows.iter().enumerate() {
            for j in i.1.iter().enumerate() {
                self.rows[j.0][i.0] = (*j.1, false);
                self.search.insert(*j.1, (i.0, j.0));
            }
        }
    }

    fn calculate_score(&self) -> usize {
        self.rows
            .iter()
            .flatten()
            .filter(|i| !i.1)
            .fold(0, |r, i| r + i.0)
    }

    /* not really needed
    // return a vec<vec<BingoCell>> of columns
    fn get_columns(&self) -> Vec<Vec<BingoCell>> {
        let mut cols: Vec<Vec<BingoCell>> = Vec::new();
        cols.resize_with(self.size, Vec::new);

        for i in self.rows.iter().enumerate() {}

        cols
    }
    */

    fn get_cell_coords(&self, content: usize) -> Option<BingoPos> {
        self.search.get(&content).copied()
    }

    fn get_cell_bingopos(&self, pos: BingoPos) -> Option<BingoCell> {
        self.get_cell(pos.0, pos.1)
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<BingoCell> {
        if let Some(r) = self.rows.get(y) {
            r.get(x).copied()
        } else {
            None
        }
    }

    fn set_cell_bingopos(&mut self, pos: BingoPos, toggle: bool) -> Result<(), String> {
        self.set_cell(pos.0, pos.1, toggle)
    }

    fn set_cell(&mut self, x: usize, y: usize, toggle: bool) -> Result<(), String> {
        if let Some(r) = self.rows.get_mut(y) {
            if let Some(c) = r.get_mut(x) {
                c.1 = toggle;
                self.check_for_bingo(x, y);

                Ok(())
            } else {
                Err("could not set cell".to_string())
            }
        } else {
            Err("could not set cell".to_string())
        }
    }

    fn check_for_bingo(&mut self, x: usize, y: usize) {
        let ix: isize = x.try_into().unwrap();
        let iy: isize = y.try_into().unwrap();

        if self.check_towards_horizontal(ix, iy) == self.size
            || self.check_towards_vertical(ix, iy) == self.size
        {
            self.has_bingo = true;
        }
    }

    fn check_towards_horizontal(&self, x: isize, y: isize) -> usize {
        if let Some(c) = self.get_cell(x as usize, y as usize) {
            if !c.1 {
                0
            } else {
                1 + self.check_towards(x, y, Direction::Left, 0)
                    + self.check_towards(x, y, Direction::Right, 0)
            }
        } else {
            0
        }
    }

    fn check_towards_vertical(&self, x: isize, y: isize) -> usize {
        if let Some(c) = self.get_cell(x as usize, y as usize) {
            if !c.1 {
                0
            } else {
                1 + self.check_towards(x, y, Direction::Up, 0)
                    + self.check_towards(x, y, Direction::Down, 0)
            }
        } else {
            0
        }
    }

    fn check_towards(&self, x: isize, y: isize, direction: Direction, amount: usize) -> usize {
        let (new_x, new_y) = direction.move_coords((x as i8, y as i8));

        // if any of this stuff is too much in one direction, we just return
        // the amount we've accumulated so far
        //
        // also: rustfmt IMMEDIATELY does C# brackets and i Die
        if (new_x < 0 || new_x > (self.size - 1).try_into().unwrap())
            || (new_y < 0 || new_y > (self.size - 1).try_into().unwrap())
        {
            return amount;
        }

        // println!("checking: ({}, {}) | amt: {}", new_x, new_y, amount);

        if let Some(c) = self.get_cell(new_x as usize, new_y as usize) {
            if c.1 {
                self.check_towards(new_x as isize, new_y as isize, direction, amount + 1)
            } else {
                amount
            }
        } else {
            amount
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn itou() {
        assert_eq!(5isize as usize, 5);

        // overflows! fun
        // assert_eq!(-5isize as usize, 0);
    }

    #[test]
    fn test_day_four() {
        let file = File::open("res/day_four.input").unwrap();
        let mut bingo = Bingo::parse_from_file(file);
        println!("{}", bingo.process_bingo());

        let file = File::open("res/day_four.input").unwrap();
        let mut least_bingo = Bingo::parse_from_file(file);
        println!("{}", least_bingo.process_least_bingo());
    }

    #[test]
    fn bingo_parse() {
        let file = File::open("res/day_four.test").unwrap();
        let mut bingo = Bingo::parse_from_file(file);

        println!("{}", bingo.process_bingo());

        let file = File::open("res/day_four.test").unwrap();
        let mut least_bingo = Bingo::parse_from_file(file);

        println!("{}", least_bingo.process_least_bingo());
    }

    #[test]
    fn bingo_table() {
        let mut table = BingoTable::new(5, 0);

        table.populate(&[
            vec![22, 13, 17, 11, 0],
            vec![8, 2, 23, 4, 24],
            vec![21, 9, 14, 16, 7],
            vec![6, 10, 3, 18, 5],
            vec![1, 12, 20, 15, 19],
        ]);
        println!("{:?}", table);

        (0..5).for_each(|i| table.set_cell(0, i, true).unwrap());
        println!("{}", table);
        println!("bingo: {}", table.has_bingo);

        (0..5).for_each(|i| table.set_cell(0, i, false).unwrap());
        table.has_bingo = false;

        (0..5).for_each(|i| table.set_cell(i, 2, true).unwrap());
        println!("{}", table);
        println!("bingo: {}", table.has_bingo);

        (0..5).for_each(|i| table.set_cell(i, 2, false).unwrap());
        table.has_bingo = false;

        (2..5).for_each(|i| table.set_cell(i, 2, true).unwrap());
        println!("{}", table);
        println!("bingo: {}", table.has_bingo);

        (2..5).for_each(|i| table.set_cell(i, 2, false).unwrap());

        let coord_set = vec![17, 23, 14, 3, 20]
            .iter()
            .map(|i| table.get_cell_coords(*i))
            .flatten()
            .collect::<Vec<BingoPos>>();

        for coord in coord_set {
            table.set_cell(coord.0, coord.1, true);
        }

        println!("{}", table);
        println!("bingo: {}", table.has_bingo);

        println!("score: {}", table.calculate_score());
    }

    #[test]
    fn directions() {
        let coordinates = (1, 1);

        assert_eq!(Direction::Up.move_coords(coordinates), (1, 2));
        assert_eq!(Direction::Down.move_coords(coordinates), (1, 0));
        assert_eq!(Direction::Left.move_coords(coordinates), (0, 1));
        assert_eq!(Direction::Right.move_coords(coordinates), (2, 1));
    }
}
