use crate::w2::hamming_distance::hamming_distance;

pub fn distance_btw_pattern_strings(pattern: &str, dna: &str) -> u64 {
    let mut distance = 0;
    let pattern_len = pattern.len();
    let dna_seq: Vec<&str> = dna.split(' ').collect();
    for i in 0..dna_seq.len() {
        let mut min_dis = dna_seq[0].len() as u64;
        for j in 0..dna_seq[0].len() - pattern_len  {
            let string = &dna_seq[i][j..j+pattern_len];
            let ham_dis = hamming_distance(pattern, string);
            if min_dis > ham_dis {
                min_dis = ham_dis;
            }
        }            
        distance += min_dis;

    }
    distance
}

#[test]
fn test_distance_btw_pattern_strings() {
    let pattern = "AAA";
    let dna = "TTACCTTAAC GATATCTGTC ACGGCGTTCG CCCTAAAGAG CGTCAGAGGT";
    let dis = distance_btw_pattern_strings(pattern, dna);

    let ans = 5;
    assert_eq!(dis, ans);
}