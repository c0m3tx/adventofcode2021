use std::collections::HashMap;
type RuleSet = HashMap<String, char>;

fn load_input(input: &str) -> (String, RuleSet) {
    let mut rules = RuleSet::new();

    let mut lines = input.lines();
    let starting = lines.next().unwrap().into();
    lines.next();

    while let Some(line) = lines.next() {
        let mut split = line.split(" -> ");
        let pattern = split.next().unwrap();
        let output = split.next().unwrap();

        rules.insert(pattern.into(), output.chars().next().unwrap());
    }

    (starting, rules)
}

fn step(input: &str, rules: &RuleSet) -> String {
    let mut last_match = false;
    let mut output: String = input
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .map(|window| {
            let key: String = [window[0], window[1]].iter().collect();
            match rules.get(&key) {
                None => {
                    last_match = false;
                    key
                }
                Some(value) => {
                    last_match = true;
                    [window[0], *value].iter().collect()
                }
            }
        })
        .collect();

    output.push(input.chars().last().unwrap());
    output
}

fn convert_input(input: &str) -> HashMap<String, u64> {
    let mut map = HashMap::new();

    input
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .for_each(|window| {
            let key: String = [window[0], window[1]].iter().collect();
            let entry = map.entry(key).or_insert(0);
            *entry += 1;
        });

    map
}

fn convert_rules(rules: &RuleSet) -> HashMap<String, Vec<String>> {
    let mut out = HashMap::new();
    rules.iter().for_each(|(key, c)| {
        let first = vec![key.chars().nth(0).unwrap(), *c];
        let second = vec![*c, key.chars().nth(1).unwrap()];
        let first: String = first.iter().collect();
        let second: String = second.iter().collect();
        out.insert(key.clone(), vec![first, second]);
    });

    out
}

fn part_1(steps: usize) -> HashMap<char, u64> {
    let input = std::fs::read_to_string("input").expect("Unable to read file");
    let (mut input, rules) = load_input(&input);
    for _ in 0..steps {
        input = step(&input, &rules);
    }

    let mut charmap = HashMap::new();
    input.chars().for_each(|c| {
        let count = charmap.entry(c).or_insert(0);
        *count += 1;
    });

    charmap
}

fn part_2(steps: usize) -> HashMap<char, u64> {
    let input = std::fs::read_to_string("input").expect("Unable to read file");
    let (input, rules) = load_input(&input);
    let last_char = input.chars().last().unwrap();

    let mut input = convert_input(&input);
    let rules = convert_rules(&rules);

    for _ in 0..steps {
        let mut new_input = HashMap::new();
        input.iter().for_each(|(key, amount)| {
            rules
                .get(key)
                .expect("Key does not exist?")
                .iter()
                .for_each(|rule| {
                    let entry = new_input.entry(rule.clone()).or_insert(0);
                    *entry += amount;
                });
        });

        input = new_input;
    }

    let mut charmap = HashMap::new();
    input.iter().for_each(|(key, amount)| {
        let c = key.chars().nth(0).unwrap();
        let count = charmap.entry(c).or_insert(0);
        *count += *amount;
    });

    let entry = charmap.entry(last_char).or_insert(0);
    *entry += 1;

    charmap
}

fn min_max(map: &HashMap<char, u64>) -> (u64, u64) {
    let min = map.values().min().unwrap();
    let max = map.values().max().unwrap();

    (*min, *max)
}

fn main() {
    let part_1 = part_1(10);
    let (min, max) = min_max(&part_1);
    println!("Part 1: {}", max - min);

    let part_2 = part_2(40);
    let (min, max) = min_max(&part_2);
    println!("Part 2: {}", max - min);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_load_input() {
        let input = "NNCB\n\nCH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C";
        let (starting, rules) = load_input(input);
        assert_eq!(starting, "NNCB");
        assert_eq!(rules.len(), 16);
    }

    #[test]
    fn test_step() {
        let input = "NNCB\n\nCH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C";
        let (starting, rules) = load_input(input);
        let step_result = step(&starting, &rules);
        assert_eq!(step_result, "NCNBCHB")
    }

    #[test]
    fn test_part_1() {
        part_1(5);
    }

    #[test]
    fn test_part_2() {
        let part_1_result = part_1(5);
        let part_2_result = part_2(5);
        assert_eq!(part_1_result, part_2_result);
    }

    #[test]
    fn test_convert_input() {
        let result = convert_input("ABBCAB");
        let mut expected = HashMap::new();
        expected.insert("AB".into(), 2);
        expected.insert("BB".into(), 1);
        expected.insert("BC".into(), 1);
        expected.insert("CA".into(), 1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_convert_rules() {
        let mut rules = HashMap::new();
        rules.insert("AB".into(), 'B');
        let result = convert_rules(&rules);
        let mut expected = HashMap::new();
        expected.insert("AB".into(), vec!["AB".into(), "BB".into()]);

        assert_eq!(result, expected);
    }
}
