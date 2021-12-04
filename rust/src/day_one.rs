pub fn count_increases(list: &[isize]) -> isize {
    let mut inc: isize = 0;
    let mut prev: Option<isize> = None;

    for i in list {
        if let Some(x) = prev {
            if *i > x {
                inc += 1;
            }
        }

        prev = Some(*i);
    }

    inc
}

pub fn count_increases_in_threes(list: &[isize]) -> isize {
    let mut inc: isize = 0;
    let mut prev: Option<isize> = None;

    let mut i = 0;
    while i + 2 < list.len() {
        let cur: isize = list[i..i + 3].iter().sum();
        if let Some(x) = prev {
            if cur > x {
                inc += 1;
            }
        }

        i += 1;
        prev = Some(cur);
    }

    inc
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_day_one() {
        let mut test_file = File::open("res/day_one.input").unwrap();
        let mut test_string: String = String::new();

        test_file.read_to_string(&mut test_string).unwrap();

        let num_vec: Vec<isize> = test_string
            .lines()
            .map(|i| i.parse::<isize>())
            .filter(|i| i.is_ok())
            .map(|i| i.unwrap())
            .collect();

        println!("part one: {}", count_increases(&num_vec));

        println!("part two: {}", count_increases_in_threes(&num_vec));
    }
}
