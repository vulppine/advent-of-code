use std::collections::HashSet;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Table<T> {
    rows: Vec<Vec<T>>,
    row_size: Option<usize>,
}

impl<T> Table<T> {
    fn new() -> Self {
        Table {
            rows: Vec::new(),
            row_size: None,
        }
    }

    // lazy, so we'll just throw isizes into this
    fn get_elem_at(&self, x: isize, y: isize) -> Option<&T> {
        if (x < 0 || x > (self.row_size.unwrap() - 1).try_into().unwrap())
            || (y < 0 || y > (self.rows.len() - 1).try_into().unwrap())
        {
            return None;
        }

        if let Some(r) = self.rows.get(y as usize) {
            r.get(x as usize)
        } else {
            None
        }
    }

    fn get_elems_around(&self, x: isize, y: isize) -> Vec<Option<(&T, (isize, isize))>> {
        let up = (x, y + 1);
        let down = (x, y - 1);
        let left = (x - 1, y);
        let right = (x + 1, y);

        vec![
            self.get_elem_at(up.0, up.1).map(|r| (r, up)),
            self.get_elem_at(down.0, down.1).map(|r| (r, down)),
            self.get_elem_at(left.0, left.1).map(|r| (r, left)),
            self.get_elem_at(right.0, right.1).map(|r| (r, right)),
        ]
    }

    fn insert_row(&mut self, row: Vec<T>) {
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
    fn from_file(file: File) -> Self {
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

fn compare_points<T: Ord>(point: &T, surrounding: &[Option<(&T, (isize, isize))>]) -> bool {
    surrounding
        .iter()
        .filter(|v| v.is_some())
        .flatten()
        .map(|v| v.0)
        .all(|x| x > point)
}

fn get_low_points<T: Ord + Clone>(table: &Table<T>) -> Vec<(&T, (usize, usize))> {
    table
        .rows
        .iter()
        .enumerate()
        .map(|r| {
            r.1.iter()
                .enumerate()
                .map(|v| {
                    (
                        v.1,
                        compare_points::<T>(
                            &v.1,
                            &table
                                .get_elems_around(v.0.try_into().unwrap(), r.0.try_into().unwrap()),
                        ),
                        (v.0, r.0),
                    )
                })
                .collect::<Vec<(&T, bool, (usize, usize))>>()
        })
        .flatten()
        .filter(|v| v.1)
        .map(|v| (v.0, v.2))
        .collect::<Vec<(&T, (usize, usize))>>()
}

struct DepthChecker {
    table: Table<u32>,
    known_points: HashSet<(isize, isize)>, // cache for known points
}

impl DepthChecker {
    fn new(table: Table<u32>) -> Self {
        Self {
            table,
            known_points: HashSet::new(),
        }
    }

    fn new_from_file(file: File) -> Self {
        Self::new(Table::<u32>::from_file(file))
    }

    fn get_basins(&mut self) -> Vec<Vec<u32>> {
        let low_points = get_low_points(&self.table)
            .iter()
            .map(|v| (*v.0, v.1))
            .collect::<Vec<(u32, (usize, usize))>>();
        let basin_map: Vec<Vec<u32>> = Vec::new();
        let mut res = Vec::new();

        for p in low_points {
            let mut basin: Vec<u32> = Vec::new();
            self.search_from_point(
                p.1 .0.try_into().unwrap(),
                p.1 .1.try_into().unwrap(),
                &mut basin,
            );

            res.push(basin);
        }

        res
    }

    // searches from a point recursively, adding
    // any new points it comes across according to
    // the rules: any 9-values are automatically
    // filtered out, any coordinates that are valid
    // are cached, and all points are input to the
    // given basin (buffer)
    fn search_from_point(&mut self, x: isize, y: isize, basin: &mut Vec<u32>) {
        if self.known_points.contains(&(x, y)) {
            return;
        }

        self.known_points.insert((x, y));
        basin.push(*self.table.get_elem_at(x, y).unwrap());
        // aslkjsdflkjsdfljsfdoiuesf
        let elems = self
            .table
            .get_elems_around(x, y)
            .iter()
            .flatten() // get all Some(v)s
            .filter(|v| *v.0 != 9) // filter out anything that isn't nine
            .map(|v| (*v.0, v.1)) // remap
            .collect::<Vec<(u32, (isize, isize))>>();

        for e in elems {
            self.search_from_point(e.1 .0, e.1 .1, basin)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_basin_counting() {
        let file = File::open("res/day_nine.test").unwrap();
        let table = Table::<u32>::from_file(file);
        let mut basin_counter = DepthChecker::new(table);

        let mut basins = basin_counter
            .get_basins()
            .iter()
            .map(|v| v.len())
            .collect::<Vec<usize>>();

        basins.sort_unstable();
        println!("{:?}", basins);

        println!(
            "{}",
            basins[basins.len() - 3..basins.len()]
                .iter()
                .product::<usize>()
        );
    }

    #[test]
    fn test_day_nine() {
        let file = File::open("res/day_nine.input").unwrap();
        let table = Table::<u32>::from_file(file);

        let res = get_low_points(&table)
            .iter()
            .fold(0, |acc, x| acc + x.0 + 1);

        println!("{}", res);

        let mut basin_counter = DepthChecker::new(table);

        let mut basins = basin_counter
            .get_basins()
            .iter()
            .map(|v| v.len())
            .collect::<Vec<usize>>();

        basins.sort_unstable();

        println!(
            "{}",
            basins[basins.len() - 3..basins.len()]
                .iter()
                .product::<usize>()
        );
    }

    #[test]
    fn test_table_counting() {
        let file = File::open("res/day_nine.test").unwrap();
        let table = Table::<u32>::from_file(file);

        println!("{:?}", get_low_points(&table));
    }

    #[test]
    fn test_table_loading() {
        let file = File::open("res/day_nine.test").unwrap();
        let table = Table::<u32>::from_file(file);

        println!("{:?}", table);
    }
}
