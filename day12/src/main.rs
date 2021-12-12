use std::collections::HashMap;

type Path = Vec<String>;

#[derive(Debug, Clone)]
struct Node {
    label: String,
    neighbors: Vec<String>,
    visited: i32,
}

impl Node {
    fn new(label: &str) -> Node {
        if label.len() == 0 {
            panic!("Node label cannot be empty");
        }

        Node {
            label: label.to_owned(),
            neighbors: vec![],
            visited: 0,
        }
    }

    fn is_large(&self) -> bool {
        self.label.chars().next().unwrap().is_uppercase()
    }
}

#[derive(Debug, Clone)]
struct Caves {
    map: HashMap<String, Node>,
    already_visited_twice_small_cave: bool,
}

impl Caves {
    fn load(input: &str) -> Self {
        let mut map = HashMap::new();
        for line in input.lines() {
            let labels: Vec<String> = line.split("-").map(|s| s.into()).collect();
            let from_label = labels[0].clone();
            let to_label = labels[1].clone();
            let from_node = map
                .entry(from_label.clone())
                .or_insert(Node::new(&from_label));
            from_node.neighbors.push(to_label.clone());

            let to_node = map.entry(to_label.clone()).or_insert(Node::new(&to_label));
            to_node.neighbors.push(from_label.clone());
        }

        Caves {
            map,
            already_visited_twice_small_cave: false,
        }
    }

    fn find_paths(mut self, node: &str, path: Path, limit: i32) -> Option<Vec<Path>> {
        let mut path = path;
        path.push(node.into());
        if self.map[node.into()].label == String::from("end") {
            return Some(vec![path]);
        }
        let visit_node = self.map.get_mut(node.into()).unwrap();
        visit_node.visited += 1;
        if visit_node.visited >= limit && !visit_node.is_large() {
            self.already_visited_twice_small_cave = true;
        }

        let child_paths: Vec<Path> = self.map[node]
            .neighbors
            .iter()
            .filter(|node| {
                let node = self.map.get(node.clone()).unwrap();
                node.label != "start"
                    && (node.is_large()
                        || (node.visited == 0)
                        || (node.visited < limit && !self.already_visited_twice_small_cave))
            })
            .filter_map(|neigh| self.clone().find_paths(&neigh, path.clone(), limit))
            .flatten()
            .collect();

        if child_paths.len() == 0 {
            return None;
        }
        Some(child_paths)
    }
}

fn part_1(caves: &Caves) -> usize {
    let paths = caves.clone().find_paths("start", vec![], 1).unwrap();
    paths.len()
}

fn part_2(caves: &Caves) -> usize {
    let paths = caves.clone().find_paths("start", vec![], 2).unwrap();
    paths.len()
}

fn main() {
    let caves = Caves::load(std::fs::read_to_string("input").unwrap().as_str());
    println!("Part 1: {}", part_1(&caves));
    println!("Part 2: {}", part_2(&caves));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cave_loading() {
        let input =
            "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc";
        let caves = Caves::load(input);
        assert_eq!(caves.map.len(), 7);
        assert_eq!(caves.map["start".into()].neighbors.len(), 3);
    }

    #[test]
    fn test_part_1() {
        let input =
            "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc";
        let caves = Caves::load(input);
        let paths = part_1(&caves);
        assert_eq!(paths, 19);
    }

    #[test]
    fn test_part_2() {
        let input =
            "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc";
        let caves = Caves::load(input);
        let paths = part_2(&caves);

        assert_eq!(paths, 103);
    }
}
