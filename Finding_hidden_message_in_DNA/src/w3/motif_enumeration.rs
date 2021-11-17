/* This fn is to extract possible motifs in different dna strings.
the algo is so simple enough to say ineffecient in practice.
At first, make a list countains possible motif patterns considering permissible hamming distance 
in any dna strings. Then, all of them are put into all possible strings in dna
to find if it is contained in all different dna seq.
If so, add it to possible motif list.
*/
use crate::w2::approx_pattern_matching::*;
use crate::w2::neighbors::neighbors;

//should not use for loop under for loop under for loop i know...
pub fn motif_enumeration(dna_set: &str, kmer: usize, permissible_dis: u64) -> Vec<String> {
    let dna_set: Vec<&str> = dna_set.split(' ').collect();
    let mut pos_patterns: Vec<String> = Vec::new();
    for dna in &dna_set {
        for i in 0..=(dna.len() - kmer) {
            let pattern= &dna[i..i + kmer];
            let pat_nei = neighbors(pattern.to_string(), permissible_dis);
            for j in pat_nei.clone() {
                if pos_patterns.contains(&j) {
                    continue;
                } else {
                    let pos_pat = j.clone();
                    pos_patterns.insert(pos_patterns.len(), pos_pat);
                } 
            }
        }
    }

    let mut freq_patterns = Vec::new();        
    for pattern in pos_patterns.clone() {    
        let mut count = 0;
        for dna in dna_set.clone() {
            let cou = count_approx_pattern(&pattern, dna, permissible_dis);
            if cou > 0 {
                count += 1; 
            }
        }
        if count > dna_set.len() as u64 - 1 {
            let true_pat = pattern.to_owned();
            freq_patterns.insert(freq_patterns.len(), true_pat);
        }
    }
    freq_patterns
}

#[test]
fn test_motif_enumeration() {
    let dnas = "ATTTGGC TGCCTTA CGGTATC GAAAATT";
    let mut freq_pats = motif_enumeration(dnas, 3, 1);
    freq_pats.sort_unstable();

    let ans = "ATA ATT GTT TTT";
    let mut ans_set: Vec<&str> = ans.split(' ').collect();
    ans_set.sort_unstable();

    assert_eq!(freq_pats, ans_set);

}