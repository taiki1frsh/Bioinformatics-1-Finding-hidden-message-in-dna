// approx means pattern matching that is condired mismatches

use crate::w2::hamming_distance::*;
use crate::w1::rev_complements::rev_complements;

#[derive(Debug, Clone)]
pub struct ApproxPattern {
    pub patterns: Vec<String>,
    pub counts: Vec<u64>,
    permissible_distance: u64,
    max_count: u64
}

impl ApproxPattern {
    pub fn new(genome: &str, k: usize, permissible_distance: u64) -> ApproxPattern {
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
                let count = count_approx_pattern(&pattern, genome, permissible_distance);
                counts.insert(counts.len(), count);
                if count > max_count {
                    max_count = count;
                }
            }
        }   
        ApproxPattern {
            patterns,
            counts,
            permissible_distance,
            max_count
        }
    }

    pub fn most_freq_approx_patterns(self) ->  Vec<String> {
        let mut freq_approx_patterns = Vec::new();
        let mut index = 0;
        for i in self.counts.clone() {
            if i == self.max_count {
                freq_approx_patterns.insert(freq_approx_patterns.len(), self.patterns.iter().nth(index).unwrap().clone());
            }
            index += 1;
        }
        freq_approx_patterns
    }   

    /* pub fn most_freq_approx_patterns_with_rev(self, genome: &str, k: usize, permissible_distance: u64) -> String{
        let mut rev_comp_genome = rev_complements(genome);
        rev_comp_genome += genome;
        rev_comp_genome
        /* let rev_pattern = ApproxPattern::new(&rev_comp_genome, k, permissible_distance);
        let mut freq_approx_pattern_with_rev = rev_pattern.most_freq_approx_patterns();
        freq_approx_pattern_with_rev */
    } */
}

pub fn approx_pattern_locations(pattern: &str, text: &str, permissible_distance: u64) -> Vec<u64> {
    let mut locations = Vec::new();
    for i in 0..text.len()- pattern.len() + 1{
        let text1 = text[i..i + pattern.len()].to_owned();
        let dis = hamming_distance(&text1, &pattern);
        if dis <= permissible_distance {
            locations.insert(locations.len(), i as u64);
        }
    }
    locations
}

pub fn count_approx_pattern(pattern: &str, genome: &str, permissible_distance: u64) -> u64 {
    let mut count = 0;
    let len_of_pattern = pattern.len();
    for i in 0..genome.len() - len_of_pattern + 1{
        let substring = &genome[i..i + len_of_pattern];
        if hamming_distance(pattern, substring) <= permissible_distance {
            count += 1;
        }
    }
    count
}

pub fn most_freq_approx_patterns_with_rev(genome: &str, k: usize, permissible_distance: u64) -> Vec<String> {
    let mut rev_comp_genome = rev_complements(genome);
    rev_comp_genome += genome;
    let approx_pattern = ApproxPattern::new(&rev_comp_genome, k, permissible_distance);
    let most_freq_approx_patterns_with_rev = approx_pattern.most_freq_approx_patterns();
    most_freq_approx_patterns_with_rev
}

#[test]
fn test_approx_pattern_matching() {
    // sample data
    let pattern = "ATTCTGGA";
    let genome = "CGCCCGAATCCAGAACGCATTCCCATATTTCGGGACCACTGGCCTCCACGGTACGGACGTCAATCAAAT";
    let permissible_distance = 3;
    let freq_approx_patterns = approx_pattern_locations(pattern, genome, permissible_distance);

    let ans = vec![6,7,26,27];
    assert_eq!(freq_approx_patterns, ans);

    let approx_pattern = ApproxPattern::new(genome, 8, permissible_distance);
    let freq_approx_patterns = approx_pattern.most_freq_approx_patterns();
    let ans2 = vec!["ATTCTGGA"];
    assert_eq!(freq_approx_patterns, ans2);
}
