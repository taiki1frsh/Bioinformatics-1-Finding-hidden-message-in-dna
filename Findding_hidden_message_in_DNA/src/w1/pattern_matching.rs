#[derive(Debug, Clone)]
pub struct Pattern {
    length: usize,
    pub patterns: Vec<String>,
    pub counts: Vec<u64>,
    max_count: u64
}

impl Pattern {
    pub fn new(genome: &str, k: usize) -> Pattern {
        let length = genome.len();
        let mut patterns = Vec::<String>::new();
        let mut counts = Vec::<u64>::new();
        let mut max_count = 0;
        
        for i in 0..length - k + 1 {
            let pattern = &genome[i..i + k];
            if patterns.iter().any(|i| i == pattern) {
                continue;
            } else {
                patterns.insert(patterns.len(), pattern.to_string());
                let count = count_pattern(&pattern, genome);
                counts.insert(counts.len(), count);
                if count > max_count {
                    max_count = count;
                }
            }
        }   
        Pattern {
            length,
            patterns,
            counts,
            max_count
        }
    }

    pub fn most_freq_patterns(&self) ->  Vec<&String> {
        let mut freq_patterns = Vec::new();
        let mut index = 0;
        for i in self.counts.clone() {
            if i == self.max_count {
                freq_patterns.insert(freq_patterns.len(), self.patterns.iter().nth(index).unwrap());
            }
            index += 1;
        }
        freq_patterns
    }   
}

pub fn count_pattern(pattern: &str, genome: &str) -> u64 {
    let mut count: u64 = 0;
    for i in 0..genome.len() - pattern.len() + 1 {
        if pattern == &genome[i..i + pattern.len()]{
            count += 1;
        }
    }
    count
}

#[test]
fn test_most_freq_pattern() {
    // sample data
    let genome = "ACGTTGCATGTCGCATGATGCATGAGAGCT";
    let k = 4;
    let pattern_map = Pattern::new(genome, k);
    let mut most_freq_patterns = pattern_map.most_freq_patterns();
    most_freq_patterns.sort_unstable();

    let mut ans = vec!["CATG", "GCAT"];
    ans.sort_unstable();
    assert_eq!(most_freq_patterns, ans);
}