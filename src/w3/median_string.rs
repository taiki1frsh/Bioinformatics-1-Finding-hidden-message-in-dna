/* fn brief explanation
Main goal is to find k-mer that is most possible motif in all dna strings.
The process is first calculate minimum d(pattern, dna) and at the end return 
the pattern have least collective point.
*/
use crate::w2::hamming_distance::*;

pub fn median_string(dnas: Vec<&str>, kmer: usize) -> String {
    let patterns = make_patterns(dnas.clone(), kmer);
    let mut min_score = 100;
    let mut min_string = String::new();

    for pattern in patterns{
        let score = calculate_min_sum_dis(pattern, dnas.clone(), kmer);
        if score < min_score {
            min_score = score;
            min_string = pattern.to_owned();
        }
    }
    min_string
}

fn calculate_min_sum_dis(pattern: &str, dnas: Vec<&str>, kmer: usize) -> u64 {
    let mut min_sum = 0;
    for dna in dnas {    
        let mut min_dis = dna.len() as u64;
        for i in 0..=dna.len() - kmer{
            let pos_motif = &dna[i..i + kmer];
            let ham_dis = hamming_distance(pattern, pos_motif);
            if ham_dis < min_dis {
                min_dis = ham_dis;
            }
        }
        min_sum += min_dis;
    }
    min_sum
}

fn make_patterns(dnas: Vec<&str>, kmer: usize) -> Vec<&str> {
    let mut patterns = Vec::new();
    for dna in dnas.clone() {
        for i in 0..=dna.len() - kmer {
            let pattern = &dna[i..i+kmer];
            if !patterns.contains(&pattern) {
                patterns.insert(patterns.len(), pattern);
            }
        }
    }
    patterns
}

#[test]
fn test_median_string() {
    let dnas = vec!["AAATTGACGCAT", "GACGACCACGTT", "CGTCAGCGCCTG", "GCTGAGCACCGG", "AGTTCGGGACAG"];
    let median_string = median_string(dnas.clone(), 3);

    let ans = "GAC".to_string();
    assert_eq!(median_string, ans);
}