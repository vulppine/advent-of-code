use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

enum UpperLower {
    Upper,
    Lower,
    Mixed,
}

fn is_upper_or_lower(s: &str) -> UpperLower {
    let mut is_upper = false;
    let mut is_lower = false;

    for c in s.chars() {
        if c.is_uppercase() {
            is_upper = true;
        }

        if c.is_lowercase() {
            is_lower = true;
        }
    }

    match (is_upper, is_lower) {
        (true, false) => UpperLower::Upper,
        (false, true) => UpperLower::Lower,
        (true, true) => UpperLower::Mixed,
        (false, false) => {
            panic!("empty string")
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum CaveNode {
    Start,
    Big(String),
    Small(String),
    End,
}

impl CaveNode {
    fn from_string(s: &str) -> Self {
        match s {
            "start" => CaveNode::Start,
            "end" => CaveNode::End,
            _ => match is_upper_or_lower(s) {
                UpperLower::Upper => CaveNode::Big(s.into()),
                UpperLower::Lower => CaveNode::Small(s.into()),
                UpperLower::Mixed => {
                    panic!("incorrect string type");
                }
            },
        }
    }

    fn is_start(&self) -> bool {
        matches!(self, CaveNode::Start)
    }

    fn is_small(&self) -> bool {
        !matches!(self, CaveNode::Big(_))
    }

    fn is_end(&self) -> bool {
        matches!(self, CaveNode::End)
    }
}

// adjacency list, yeah, yeah
#[derive(Debug)]
struct CaveSystem {
    caves: HashMap<CaveNode, HashSet<CaveNode>>,
}

impl CaveSystem {
    pub fn new() -> Self {
        Self {
            caves: HashMap::new(),
        }
    }

    pub fn from_file(file: File) -> Self {
        let mut caves = Self::new();

        let lines = BufReader::new(file).lines().flatten();

        for l in lines {
            caves.insert_from_string(l);
        }

        caves
    }

    pub fn verify(&self) -> bool {
        self.caves.get(&CaveNode::End).is_some() && self.caves.get(&CaveNode::Start).is_some()
    }

    pub fn insert_from_string(&mut self, s: String) {
        let (cave_n, cave_m) = {
            let mut split = s.split('-');

            let n_str = split.next().unwrap();
            let m_str = split.next().unwrap();

            if split.count() != 0 {
                panic!("incorrect string format");
            }

            let n = CaveNode::from_string(n_str);
            let m = CaveNode::from_string(m_str);

            (n, m)
        };

        self.insert_cave_pair(cave_n, cave_m);
    }

    fn insert_cave_pair(&mut self, cave_n: CaveNode, cave_m: CaveNode) {
        self.register_cave_edge(cave_n.clone(), cave_m.clone());
        self.register_cave_edge(cave_m, cave_n);
    }

    fn register_cave_edge(&mut self, cave_a: CaveNode, cave_b: CaveNode) {
        if let Some(node) = self.caves.get_mut(&cave_a) {
            node.insert(cave_b);
        } else {
            let mut set = HashSet::new();
            set.insert(cave_b);
            self.caves.insert(cave_a, set);
        }
    }
}

// PathTraverser. There can easily be more than one of these
// per check, so we'll only allocate hash sets and cave path
// storage.
#[derive(Clone)]
struct PathTraverser<'c> {
    small_caves: HashMap<&'c CaveNode, u8>, // store all the small caves visited so far, that way we don't revisit
    cave_path: Vec<&'c CaveNode>, // push into the cave path (also borrowed, avoid cloning)
    cave_system: &'c CaveSystem,
    max_small_cave_visits: bool,
}

impl<'c> PathTraverser<'c> {
    fn new(system: &'c CaveSystem) -> Self {
        Self {
            small_caves: HashMap::new(),
            cave_path: Vec::new(),
            cave_system: system,
            max_small_cave_visits: false,
        }
    }

    fn explore_from_start(&mut self) -> Vec<Vec<&'c CaveNode>> {
        if !self.cave_system.verify() {
            panic!("cannot explore system without a start or end");
        }

        let mut result_set = Vec::new();

        self.explore_from(&CaveNode::Start, &mut result_set);

        result_set
    }

    fn explore_from(&mut self, node: &'c CaveNode, result_set: &mut Vec<Vec<&'c CaveNode>>) {
        self.cave_path.push(node);

        if node.is_small() {
            if !self.max_small_cave_visits {
                if let Some(c) = self.small_caves.get_mut(node) {
                    *c += 1;

                    if *c >= 2 {
                        self.max_small_cave_visits = true;
                    }
                } else {
                    self.small_caves.insert(node, 1);
                }
            } else {
                self.small_caves.insert(node, 1);
            }
        }

        if node.is_end() {
            result_set.push(self.cave_path.clone());

            return;
        }

        let paths = self.cave_system.caves.get(node).unwrap(); // it would be an error otherwise

        self.traverse_paths(paths, result_set);
    }

    fn traverse_paths(
        &mut self,
        caves: &'c HashSet<CaveNode>,
        result_set: &mut Vec<Vec<&'c CaveNode>>,
    ) {
        let filter: Box<dyn Fn(&&CaveNode) -> bool> = if self.max_small_cave_visits {
            Box::new(|c: &&CaveNode| self.small_caves.get(*c).is_none() && !c.is_start())
        } else {
            Box::new(|c| !c.is_start())
        };

        for c in caves.iter().filter(|c| filter(c)) {
            // create a new pathfinder
            let mut new_traverser = self.clone();
            // start recursively exploring from
            // this given pathfinder
            new_traverser.explore_from(c, result_set);
        }

        // implicitly returns if no caves can be explored
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_day_twelve() {
        let file = File::open("res/day_twelve.input").unwrap();
        let cave_system = CaveSystem::from_file(file);

        let mut traverser = PathTraverser::new(&cave_system);

        let res = traverser.explore_from_start();

        println!("path count: {}", res.len());
    }

    #[test]
    fn test_from_file() {
        let file = File::open("res/day_twelve.test").unwrap();
        let mut cave_system = CaveSystem::from_file(file);
        let mut traverser = PathTraverser::new(&cave_system);

        let res = traverser.explore_from_start();

        println!("path count: {}", res.len());
    }

    #[test]
    fn test_cave_traversal() {
        let mut cave_system = CaveSystem::new();

        cave_system.insert_from_string("start-A".into());
        cave_system.insert_from_string("start-b".into());
        cave_system.insert_from_string("A-c".into());
        cave_system.insert_from_string("A-b".into());
        cave_system.insert_from_string("b-d".into());
        cave_system.insert_from_string("A-end".into());
        cave_system.insert_from_string("b-end".into());

        let mut traverser = PathTraverser::new(&cave_system);

        let res = traverser.explore_from_start();

        for p in &res {
            println!("{:?}", p);
        }

        println!("path total: {}", res.len());
    }

    #[test]
    fn test_node_parsing() {
        assert_eq!(CaveNode::Start, CaveNode::from_string("start"));
        assert_eq!(CaveNode::End, CaveNode::from_string("end"));
        assert_eq!(CaveNode::Big("A".into()), CaveNode::from_string("A"));
        assert_eq!(CaveNode::Small("b".into()), CaveNode::from_string("b"));
    }

    #[test]
    fn test_cave_construction() {
        let mut cave_system = CaveSystem::new();

        cave_system.insert_cave_pair(CaveNode::Big("test".into()), CaveNode::Small("test".into()));
        cave_system.insert_cave_pair(
            CaveNode::Big("test_2".into()),
            CaveNode::Small("test_2".into()),
        );

        println!("{:?}", cave_system);
    }
}
