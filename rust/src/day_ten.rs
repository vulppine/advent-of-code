use std::collections::HashMap;
use std::collections::LinkedList;

struct ChunkParser {
    chunk_map: HashMap<char, &'static dyn Chunk>,
    queue: Vec<&'static dyn Chunk>,
}

// chunks: the parser uses these, and
// throws the current character against
// whatever chunk is created
//
// if a closer matches, then the chunk
// is closed and popped off
trait Chunk {
    fn get_closer(&self) -> char;

    fn is_closer(&self, c: char) -> bool;
}

struct BracketChunk {}

impl Chunk for BracketChunk {
    fn get_closer(&self) -> char {
        '}'
    }

    fn is_closer(&self, c: char) -> bool {
        c == '}'
    }
}

struct ParenthesesChunk {}

impl Chunk for ParenthesesChunk {
    fn get_closer(&self) -> char {
        ')'
    }

    fn is_closer(&self, c: char) -> bool {
        c == ')'
    }
}

struct AngleChunk {}

impl Chunk for AngleChunk {
    fn get_closer(&self) -> char {
        ']'
    }

    fn is_closer(&self, c: char) -> bool {
        c == ']'
    }
}

struct ArrowChunk {}

impl Chunk for ArrowChunk {
    fn get_closer(&self) -> char {
        '>'
    }

    fn is_closer(&self, c: char) -> bool {
        c == '>'
    }
}

fn create_chunk(c: char) -> Box<dyn Chunk> {
    match c {
        '{' => Box::new(BracketChunk {}),
        '(' => Box::new(ParenthesesChunk {}),
        '[' => Box::new(AngleChunk {}),
        _ => {
            panic!("not supported")
        }
    }
}

impl ChunkParser {
    fn new() -> Self {
        // oop in Rust? it's more likely than you think
        let mut chunk_map: HashMap<char, &'static dyn Chunk> = HashMap::new();

        chunk_map.insert('{', &BracketChunk {});
        chunk_map.insert('(', &ParenthesesChunk {});
        chunk_map.insert('[', &AngleChunk {});
        chunk_map.insert('<', &ArrowChunk {});

        Self {
            chunk_map,
            queue: Vec::new(),
        }
    }

    fn push_from_char(&mut self, c: char) -> bool {
        if let Some(ch) = self.chunk_map.get(&c) {
            self.queue.push(*ch);
            true
        } else {
            false
        }
    }

    fn parse_from_char(&mut self, c: char) -> bool {
        if let Some(ch) = self.queue.last() {
            if ch.is_closer(c) {
                self.queue.pop();

                true
            } else {
                self.push_from_char(c)
            }
        } else {
            self.push_from_char(c)
        }
    }

    // does not trim for you, assumes you've sent in a trimmed line
    fn parse_line(&mut self, l: &str) -> Result<bool, char> {
        self.queue.clear();

        for b in l.bytes() {
            if !self.parse_from_char(b as char) {
                return Err(b as char);
            }
        }

        Ok(self.queue.is_empty())
    }

    // only works if the last line was not auto-completed, and
    // did not result in an error
    fn auto_complete(&self) -> String {
        let mut res = String::new();
        for c in self.queue.iter().rev() {
            res.push(c.get_closer());
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn test_day_ten() {
        let file = File::open("res/day_ten.input").unwrap();
        let lines = BufReader::new(file).lines();
        let mut parser = ChunkParser::new();

        let mut err_chars: Vec<char> = Vec::new();
        let mut autocomplete: Vec<String> = Vec::new();
        for l in lines {
            match parser.parse_line(&l.unwrap()) {
                Err(c) => err_chars.push(c),
                Ok(r) => {
                    if !r {
                        autocomplete.push(parser.auto_complete())
                    }
                }
            }
        }

        let err_res = err_chars.iter().fold(0, |acc, x| {
            acc + match x {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => {
                    panic!("incorrect input")
                }
            }
        });

        let mut auto_res = autocomplete
            .iter()
            .map(|l| {
                l.chars().fold(0, |acc, x| {
                    (acc * 5)
                        + match x {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => {
                                panic!("incorrect input")
                            }
                        }
                })
            })
            .collect::<Vec<usize>>();

        auto_res.sort_unstable();

        println!("{}", err_res);
        println!("{}", auto_res[((auto_res.len() + 1) / 2) - 1]);
    }

    #[test]
    fn test_scoring() {
        println!(
            "{}",
            ")}>]})".chars().fold(0, |acc, x| {
                (acc * 5)
                    + match x {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => {
                            panic!("incorrect input")
                        }
                    }
            })
        );
    }

    #[test]
    fn test_autocomplete() {
        let mut parser = ChunkParser::new();
        println!("{:?}", parser.parse_line("<[({"));
        println!("{:?}", parser.auto_complete());
    }

    #[test]
    fn test_parse() {
        let mut parser = ChunkParser::new();
        println!("{:?}", parser.parse_line("<>"));
    }
}
