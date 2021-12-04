use std::cmp::Ordering;
use std::iter::Iterator;

struct BitIter {
    num: usize,
    cur_bit: u32,
}

impl Iterator for BitIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_bit == usize::BITS {
            None
        } else {
            let res = Some(get_nth_bit_in_word(self.num, self.cur_bit as usize));
            self.cur_bit += 1;

            res
        }
    }
}

impl BitIter {
    fn new(num: usize) -> Self {
        BitIter { num, cur_bit: 0 }
    }
}

#[derive(Debug)]
struct BitCount(pub usize, pub usize); // 0 is 0s, 1 is 1s

impl BitCount {
    fn add_zero(&mut self) {
        self.0 += 1;
    }

    fn add_one(&mut self) {
        self.1 += 1;
    }

    fn get_common_bits(count_vec: &[Self]) -> usize {
        let mut cur_bit = count_vec.len();
        let mut res = 0;
        for i in count_vec {
            let new_bit = if i.0 < i.1 { 1 } else { 0 };
            res |= new_bit << (cur_bit - 1);
            cur_bit -= 1;
        }

        res
    }

    fn get_least_common_bits(count_vec: &[Self]) -> usize {
        let shift = usize::BITS - count_vec.len() as u32;
        (!Self::get_common_bits(count_vec) << shift) >> shift
    }
}

fn get_nth_bit_in_word(word: usize, n: usize) -> u8 {
    ((word >> n) & 1) as u8
}

// given some vector of words, count the amount of bits per
// bit field L>R : i.e., counting 5 bits would count b4, b3, b2, b1, b0
// in each word respectively, returning a vec of len 5
fn count_bits_per_field(
    word_vec: &[usize],
    field_count: usize,
    buf: &mut [BitCount], // caller must clear
) {
    for word in word_vec {
        // clippy kept complaining about 0..field_count,
        // so i just turned it into an iterator
        for i in BitIter::new(*word).enumerate().take(field_count) {
            match i.1 {
                0 => buf[i.0].add_zero(),
                1 => buf[i.0].add_one(),
                _ => {
                    panic!("bitmasking failed");
                }
            };
        }
    }
}

// this one is *really* ugly - there's probably a better way to do this

fn bit_filter(iter: Box<impl Iterator>) -> Box<impl Iterator> {
    Box::new(iter.take_while(|i| true))
}

// named against the challenge desc
// because i don't know if this is a
// mathematical property
// word_vec are the words to use, field_count is the amount of bits to care about
fn get_oxy_rating(mut word_vec: Vec<usize>, field_count: usize) -> usize {
    let mut current_bit = field_count - 1;
    let mut buf: Vec<BitCount> = Vec::new();
    while word_vec.len() != 1 {
        buf.clear();
        buf.resize_with(field_count, || BitCount(0, 0));
        let bit_count = count_bits_per_field(&word_vec, field_count, &mut buf);

        let common_bit = match buf[current_bit].0.cmp(&buf[current_bit].1) {
            Ordering::Less => 1,
            Ordering::Greater => 0,
            Ordering::Equal => 1,
        };

        word_vec = word_vec
            .into_iter()
            .filter(|i| get_nth_bit_in_word(*i, current_bit) != common_bit)
            .collect::<Vec<usize>>();

        println!("{:?}", word_vec);

        // ??? why
        if current_bit == 0 {
            break;
        }

        current_bit -= 1;
    }

    word_vec[0]
}

fn get_co2_rating(mut word_vec: Vec<usize>, field_count: usize) -> usize {
    let mut current_bit = field_count - 1;
    let mut buf: Vec<BitCount> = Vec::new();
    while word_vec.len() != 1 {
        buf.clear();
        buf.resize_with(field_count, || BitCount(0, 0));
        let bit_count = count_bits_per_field(&word_vec, field_count, &mut buf);

        let common_bit = match buf[current_bit].0.cmp(&buf[current_bit].1) {
            Ordering::Less => 0,
            Ordering::Greater => 1,
            Ordering::Equal => 0,
        };

        word_vec = word_vec
            .into_iter()
            .filter(|i| get_nth_bit_in_word(*i, current_bit) != common_bit)
            .collect::<Vec<usize>>();

        // ??? why
        if current_bit == 0 {
            break;
        }

        current_bit -= 1;
    }

    word_vec[0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_bit_counting() {
        let mut buf: Vec<BitCount> = Vec::new();
        buf.resize_with(12, || BitCount(0, 0));
        count_bits_per_field(
            &[usize::from_str_radix("110001010111", 2).unwrap()],
            12,
            &mut buf,
        );
        let mcb = BitCount::get_common_bits(&buf);
        let lcb = BitCount::get_least_common_bits(&buf);

        println!("{:12b}, {:12b}", mcb, lcb);
    }

    #[test]
    fn test_day_three() {
        let mut test_file = File::open("res/day_three.input").unwrap();
        let mut test_string: String = String::new();

        test_file.read_to_string(&mut test_string).unwrap();

        let mut peek = test_string.lines().peekable();
        let field_count = peek.peek().unwrap().len();

        println!("field_count: {}", field_count);

        let num_vec: Vec<usize> = test_string
            .lines()
            .map(|i| usize::from_str_radix(i, 2).unwrap())
            .collect();

        let mut buf: Vec<BitCount> = Vec::new();
        buf.resize_with(12, || BitCount(0, 0));
        count_bits_per_field(&num_vec, field_count, &mut buf);
        let mcb = BitCount::get_common_bits(&buf);
        let lcb = BitCount::get_least_common_bits(&buf);

        println!("mcb: {:12b}", mcb);
        println!("lcb: {:12b}", lcb);
        println!("mcb * lcb: {}", mcb * lcb);

        let oxy_rating = get_oxy_rating(num_vec.clone(), field_count);
        let co2_rating = get_co2_rating(num_vec.clone(), field_count);

        println!("{}", oxy_rating);
        println!("{}", co2_rating);

        println!("{}", oxy_rating * co2_rating);
    }
}
