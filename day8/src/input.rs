pub struct Input {
    pub patterns: Vec<String>,
    pub digits: Vec<String>,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let line: Vec<&str> = s.split(" | ").collect();
        let patterns = line[0].split_whitespace().map(|s| s.into()).collect();
        let digits = line[1].split_whitespace().map(|s| s.into()).collect();
        Input { patterns, digits }
    }
}
