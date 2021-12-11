use std::cmp::Ordering;
use std::collections::HashMap;

// Some of the functions were never used in here - I've left them in here,
// in case I ever come back to this and decide to do this differently.

const SEGMENT_A: u8 = 1;
const SEGMENT_B: u8 = 1 << 1;
const SEGMENT_C: u8 = 1 << 2;
const SEGMENT_D: u8 = 1 << 3;
const SEGMENT_E: u8 = 1 << 4;
const SEGMENT_F: u8 = 1 << 5;
const SEGMENT_G: u8 = 1 << 6;

fn new_segment_map() -> HashMap<char, u8> {
    let mut r: HashMap<char, u8> = HashMap::new();

    r.insert('a', SEGMENT_A);
    r.insert('b', SEGMENT_B);
    r.insert('c', SEGMENT_C);
    r.insert('d', SEGMENT_D);
    r.insert('e', SEGMENT_E);
    r.insert('f', SEGMENT_F);
    r.insert('g', SEGMENT_G);

    r
}

// as segments, not as constructed pieces
const ONE: u8 = SEGMENT_C | SEGMENT_F;
const TWO: u8 = SEGMENT_A | SEGMENT_C | SEGMENT_D | SEGMENT_E | SEGMENT_G;
const THREE: u8 = SEGMENT_A | SEGMENT_C | SEGMENT_D | SEGMENT_F | SEGMENT_G;
const FOUR: u8 = SEGMENT_B | SEGMENT_C | SEGMENT_D | SEGMENT_F;
const FIVE: u8 = SEGMENT_A | SEGMENT_B | SEGMENT_D | SEGMENT_F | SEGMENT_G;
const SIX: u8 = SEGMENT_A | SEGMENT_B | SEGMENT_D | SEGMENT_E | SEGMENT_F | SEGMENT_G;
const SEVEN: u8 = SEGMENT_A | SEGMENT_C | SEGMENT_F;
const EIGHT: u8 = SEGMENT_A | SEGMENT_B | SEGMENT_C | SEGMENT_D | SEGMENT_E | SEGMENT_F | SEGMENT_G;
const NINE: u8 = SEGMENT_A | SEGMENT_B | SEGMENT_C | SEGMENT_D | SEGMENT_F | SEGMENT_G;
const ZERO: u8 = SEGMENT_A | SEGMENT_B | SEGMENT_C | SEGMENT_E | SEGMENT_F | SEGMENT_G;

fn display_as_int(display: u8) -> usize {
    println!("{:08b}", display);
    println!("{:08b}", THREE);

    match display {
        ZERO => 0,
        ONE => 1,
        TWO => 2,
        THREE => 3,
        FOUR => 4,
        FIVE => 5,
        SIX => 6,
        SEVEN => 7,
        EIGHT => 8,
        NINE => 9,
        _ => {
            panic!("unsupported number")
        }
    }
}

// from day 3
struct BitIter {
    num: u8,
    cur_bit: u32,
}

impl Iterator for BitIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_bit == u8::BITS {
            None
        } else {
            let res = Some(Self::get_nth_bit_in_word(self.num, self.cur_bit as u32));
            self.cur_bit += 1;

            res
        }
    }
}

impl BitIter {
    fn get_nth_bit_in_word(word: u8, n: u32) -> u8 {
        ((word >> n) & 1) as u8
    }

    fn new(num: u8) -> Self {
        BitIter { num, cur_bit: 0 }
    }
}

#[derive(Debug)]
struct SegmentDisplay {
    // segment character to actual segment as u8
    segments: HashMap<char, u8>,
    // what the actual char->display segments are
    known_segments: HashMap<char, u8>,
}

impl SegmentDisplay {
    fn new() -> Self {
        SegmentDisplay {
            // segments: HashMap::new(),
            segments: new_segment_map(),
            known_segments: HashMap::new(),
        }
    }

    // remove specific characters from a string
    fn xor_string(str_one: &str, str_two: &str) -> String {
        String::from_utf8(
            str_one
                .as_bytes()
                .iter()
                .filter(|b| !str_two.contains(**b as char))
                .copied()
                .collect::<Vec<u8>>(),
        )
        .unwrap()
    }

    // counts the characters per string
    fn count_chars_in_strings(strs: &[&str], chars: &str) -> Vec<(char, usize)> {
        let mut res = Vec::new();

        for c in chars.chars() {
            let mut count = 0;
            for s in strs {
                if s.contains(c) {
                    count += 1;
                }
            }

            res.push((c, count));
        }

        res
    }

    fn string_to_number(&self, s: &str) -> usize {
        display_as_int(s.chars().fold(0, |acc, x| {
            println!("{:08b}", acc);
            acc + self.known_segments.get(&x).unwrap()
        }))
    }

    /*
    // splits a u8 into a vec, where r.1 is the original
    // bit in the u8, and r.0 is the location of that
    // bit
    fn split_apart_u8(mut input: u8) -> Vec<(usize, u8)> {
        BitIter::new(input)
            .enumerate()
            .filter(|v| v.1 == 1)
            .map(|v| (v.0, 1 << v.0))
            .collect::<Vec<(usize, u8)>>()
    }

    // this is just get_nth_bit??? maybe a util module would
    // be useful here
    fn is_segment_in_loc(mut disp: u8, loc: u8) -> bool {
        ((disp >> loc) & 1) > 0
    }

    // counts how many times two segments appear in each
    // number as listed in nums
    //
    // r.0 is the segment, r.1 is the count in nums
    fn count_two_segments(segs: u8, nums: &[u8]) -> Vec<(u8, usize)> {
        println!("counting seg split");
        println!("{:08b}", segs);
        let split_segs = Self::split_apart_u8(segs);
        println!("{:?}", split_segs);
        if split_segs.len() > 2 {
            panic!("malformed input");
        }

        let mut res: Vec<(u8, usize)> = Vec::new();
        for c in &split_segs {
            let segcount = nums
                .iter()
                .filter(|s| Self::is_segment_in_loc(**s, c.0 as u8))
                .count();
            res.push((c.1, segcount));
        }

        res
    }
    */

    fn construct(&mut self, mut segments: &mut [&str]) {
        segments.sort_by(|a, b| a.len().cmp(&b.len()));

        /*
        for i in segments[9].chars().enumerate() {
            self.segments.insert(i.1, 1 << i.0);
        }
        */

        /*
        let segs = segments
            .iter()
            .map(|v| self.string_to_display(v))
            .collect::<Vec<u8>>();
        */

        /*
        for s in &segs {
            println!("{:08b}", s);
        }
        */

        // all unknown characters that we care about at the moment
        let mut unknowns: Vec<char> = Vec::new();

        // at this point:
        // 0 : one
        // 1 : seven
        // 2 : four

        let mut acf_seg_str = segments[1].to_string();
        acf_seg_str.push_str(segments[0]);

        let a_seg_count = elem_count_in_vec(acf_seg_str.as_bytes());
        println!("a_seg_count: {:?}", a_seg_count);

        let a_seg_char = *a_seg_count
            .iter()
            .filter(|v| v.1 <= 1)
            .map(|v| v.0)
            .next()
            .unwrap() as char;

        println!("a_seg_char: {}", a_seg_char);
        self.known_segments.insert(a_seg_char, SEGMENT_A);

        /*
        // this is the 'a' segment
        let a_seg = self.string_to_display(segments[0]) ^ self.string_to_display(segments[1]);
        println!("a_seg: {:08b}", a_seg);
        // the equivalent of segment A
        self.charsegs.insert(a_seg, SEGMENT_A);
        */

        // this is segment bd, aka 'the l from four'
        // this will be useful later, but we don't
        // currently know what exactly the segment
        // positions are

        // c and f are both located here, but we don't know which one is which
        // so we have to process both of them by checking every display
        // output and counting
        // let cf_split = Self::split_apart_u8(self.string_to_display(segments[0]));

        /*
        let mut cf_res: Vec<(usize, usize)> = Vec::new();
        for c in &cf_split {
            let segcount = segs
                .iter()
                .filter(|s| Self::is_segment_in_loc(**s, c.0 as u8))
                .count();
            cf_res.push((c.0, segcount));
        }

        if cf_res.len() > 2 {
            panic!("malformed input");
        }
        */

        // let cf_res = Self::count_two_segments(self.string_to_display(segments[0]), &segs);
        let cf_res = Self::count_chars_in_strings(&segments, segments[0]);
        println!("{:?}", cf_res);

        match cf_res[0].1.cmp(&cf_res[1].1) {
            Ordering::Less => {
                self.known_segments.insert(cf_res[0].0, SEGMENT_C);
                self.known_segments.insert(cf_res[1].0, SEGMENT_F);
            }
            Ordering::Greater => {
                self.known_segments.insert(cf_res[1].0, SEGMENT_C);
                self.known_segments.insert(cf_res[0].0, SEGMENT_F);
            }
            Ordering::Equal => {
                panic!("malformed input");
            }
        };

        /*
        // eg formed by notting 4 and segment a put together (and 1 << 7)
        let eg_segs = !(self.string_to_display(segments[2])
            | self.known_segments.get(&'a').unwrap()
            | 1 << 7);

        let eg_res = Self::count_two_segments(eg_segs, &segs);
        */

        let mut eg_segs = segments[2].to_string();
        eg_segs.push(a_seg_char);
        let eg_res =
            Self::count_chars_in_strings(&segments, &Self::xor_string(segments[9], &eg_segs));
        println!("{:?}", eg_res);

        match eg_res[0].1.cmp(&eg_res[1].1) {
            Ordering::Less => {
                self.known_segments.insert(eg_res[0].0, SEGMENT_E);
                self.known_segments.insert(eg_res[1].0, SEGMENT_G);
            }
            Ordering::Greater => {
                self.known_segments.insert(eg_res[1].0, SEGMENT_E);
                self.known_segments.insert(eg_res[0].0, SEGMENT_G);
            }
            Ordering::Equal => {
                panic!("malformed input");
            }
        }

        let mut not_bd = Self::xor_string(segments[9], &eg_segs);
        not_bd.push(a_seg_char);
        not_bd.push_str(segments[0]);

        // bd formed by isolating segments c/f, then xoring with 4 in order to get b/d

        let bd_res =
            Self::count_chars_in_strings(&segments, &Self::xor_string(segments[9], &not_bd));
        println!("{:?}", bd_res);

        match bd_res[0].1.cmp(&bd_res[1].1) {
            Ordering::Less => {
                self.known_segments.insert(bd_res[0].0, SEGMENT_B);
                self.known_segments.insert(bd_res[1].0, SEGMENT_D);
            }
            Ordering::Greater => {
                self.known_segments.insert(bd_res[1].0, SEGMENT_B);
                self.known_segments.insert(bd_res[0].0, SEGMENT_D);
            }
            Ordering::Equal => {
                panic!("malformed input");
            }
        }
        // let cf_result = cf_split.map(|v|

        /*
        // insert eight, which has all seven segments
        self.numbers.insert(8, segments[segments.len() - 1].into());

        // insert 1, which has two
        self.numbers.insert(1, segments[0].into());

        // insert 7, which has three
        self.numbers.insert(7, segments[1].into());

        let mut one_seven_segs = segments[0].to_string();
        one_seven_segs.push_str(segments[1]);
        let seg_count = elem_count_in_vec(one_seven_segs.as_bytes());

        self.segments.insert(*seg_count[0].0 as char, SEGMENT_A);
        for c in seg_count[1..].iter() {
            unknowns.push(*c.0 as char);
        }
        */

        /*
        let mut bd_segs = segments[2]
            .as_bytes()
            .iter()
            .filter(|v| *v != seg_count[1].0 || *v != seg_count[2].0);
            */

        /*
        segments = &mut segments[3..8];
        let e_count = 0;
        let g_count = 0;
        for s in segments {}
        */

        // the rest of the segments
    }
}

// gets the amount of elements in some vec<T>
fn elem_count_in_vec<'a, T: Ord + Clone>(vec: &'a [T]) -> Vec<(&'a T, usize)> {
    vec.iter().fold(Vec::new(), |mut acc, x| {
        if let Ok(v) = acc.binary_search_by(|v| v.0.cmp(x)) {
            acc[v].1 += 1;
        } else {
            acc.push((x, 1));
            acc.sort_by(|a, b| a.0.cmp(b.0));
        }

        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn test_day_eight() {
        let file = File::open("res/day_eight.input").unwrap();
        let lines = BufReader::new(file).lines();

        let res = lines.fold(0, |acc, l| {
            let line = l.unwrap();
            let mut parts = line
                .split('|')
                .map(|s| s.trim().split(' ').collect::<Vec<&str>>())
                .collect::<Vec<Vec<&str>>>();

            let mut segment_map = SegmentDisplay::new();
            segment_map.construct(&mut parts[0]);

            let res = parts[1].iter().rev().enumerate().fold(0, |acc, x| {
                if x.0 == 0 {
                    acc + segment_map.string_to_number(x.1)
                } else {
                    acc + (segment_map.string_to_number(x.1) * 10usize.pow(x.0 as u32))
                }
            });

            acc + res
        });

        println!("{}", res);
    }

    #[test]
    fn test_str_sort() {
        let mut test_input = [
            "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "eafb", "cagedb", "ab",
        ];
        test_input.sort_by(|a, b| a.len().cmp(&b.len()));
        println!("{:?}", test_input);
    }

    #[test]
    fn test_construction() {
        let mut segment_map = SegmentDisplay::new();

        let mut test_input = [
            "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab",
        ];
        test_input.sort();
        println!("{:?}", test_input);

        segment_map.construct(&mut test_input);

        for s in &segment_map.known_segments {
            println!("{}: {:08b}", s.0, s.1);
        }

        println!("{}", segment_map.string_to_number("cdfeb"));
        println!("{}", segment_map.string_to_number("fcadb"));
        println!("{}", segment_map.string_to_number("cdfeb"));
        println!("{}", segment_map.string_to_number("cdbaf"));
    }

    #[test]
    fn str_display() {
        let mut segment_map = SegmentDisplay::new();
        // println!("{:08b}", segment_map.string_to_display("ab"));
    }

    #[test]
    fn u8_splitting() {
        let res = BitIter::new(1).enumerate().collect::<Vec<(usize, u8)>>();

        println!("{:?}", res);
    }

    // largely not actual parsing
    #[test]
    fn test_part_one() {
        let file = File::open("res/day_eight.input").unwrap();
        let lines = BufReader::new(file).lines();

        let res = lines.fold(0, |acc, l| {
            println!("{:?}", l);
            acc + l
                .unwrap()
                .split('|')
                .nth(1)
                .unwrap()
                .trim()
                .split(' ')
                .filter(|v| v.len() == 2 || v.len() == 3 || v.len() == 4 || v.len() == 7)
                .count()
        });

        println!("{}", res);
    }

    #[test]
    fn elem_count() {
        println!(
            "{:?}",
            elem_count_in_vec(&[1, 3, 1, 2, 2, 3, 6, 2, 1, 4, 5, 5])
        );
        println!(
            "{:?}\n{:?}",
            elem_count_in_vec(&["d", "a", "b", "a", "b"]),
            elem_count_in_vec(&["a", "b"])
        );
    }

    #[test]
    fn segment_consts() {
        println!("{:08b}", SEGMENT_A);
        println!("{:08b}", SEGMENT_B);
        println!("{:08b}", SEGMENT_C);
        println!("{:08b}", SEGMENT_D);
        println!("{:08b}", SEGMENT_E);
        println!("{:08b}", SEGMENT_F);
        println!("{:08b}", SEGMENT_G);
    }
}
