use std::collections::{BTreeMap, HashMap};

/* Fn explanation from ls 158 step 9
This first take matrix profile as an argument calculated previously 
and second note the score the probability of each kmer.
In the end, return the kmer has the highest score.
*/
pub fn median_string_by_probability(seq: &str, kmer: usize, matrix: Vec<Vec<f64>>) -> &str {
    //matrix size is 4*kmer 
    // initiate values
    let mut map = HashMap::new();
    map.insert('A', 0);
    map.insert('C', 1);
    map.insert('G', 2);
    map.insert('T', 3);
    let mut motif = "";
    let mut max_score = 0_f64;

    // calculate probability and pick the one.
    for i in 0..=seq.len() - kmer {
        let kseq = &seq[i..i+kmer];
        let mut score = 1_f64;
        let mut count = 0;
        for i in kseq.chars() {
            score = score * matrix[map[&i]][count];
            count += 1;
        }
        
        if score > max_score {
            max_score = score;
            motif = kseq;
        }
    }
    if motif.len() == 0 {
        motif = &seq[0..kmer];
    }
    motif
}

// hand motif candidates into this fn to make a matrix for profile
// as vec<&str>
pub fn profile_matrix(motifs: Vec<&str>) -> Vec<Vec<f64>> {
    // initiate values
    let mut matrix: Vec<Vec<f64>> = vec![vec![0f64; motifs[0].len()]; 4];
    let motif_number = &motifs.len();
    let mut motif_table: Vec<Vec<&str>> = vec![vec![""; 1]; *motif_number];
    let mut count = 0;
    for i in motifs.clone() {
        motif_table[count][0] = i;
        count += 1;
    }

    let len_table = motif_table.len();
    for i in 0..motif_table[0][0].len() {
        let mut char = BTreeMap::new();
        char.insert('A', 0);
        char.insert('C', 1);
        char.insert('G', 2);
        char.insert('T', 3);
        let mut num: Vec<f64> = vec![1.0, 1.0, 1.0, 1.0];

        for j in 0..len_table {
            let text = motif_table[j][0];
            num[char[&text.chars().nth(i).unwrap()]] += 1.0;
        }
        for k in 0..num.len() {
            matrix[k][i] = num[k] / (len_table as f64 + 4.0);
        }    
    }
    matrix
}

#[test]
fn test_probability_profile() {
    let seq = "ACCTGTTTATTGCCTAAGTTCCGAACAAACCCAATATAGCCCGAGGGCCT";
    let kmer = 5;
    let matrix = vec![vec![0.2, 0.2, 0.3, 0.2, 0.3], vec![0.4, 0.3, 0.1, 0.5, 0.1], vec![0.3, 0.3, 0.5, 0.2, 0.4], vec![0.1, 0.2, 0.1, 0.1, 0.2]];
    let motif = median_string_by_probability(seq, kmer, matrix);

    let ans  = "CCGAG";
    assert_eq!(motif, ans);
}

#[test]
fn test_profile_matrix() {
    let motifs = vec!["AT", "AT", "CT"];
    let matrix = profile_matrix(motifs);

    let ans = vec![vec![2.0 / 3.0, 0.0], vec![1.0 / 3.0, 0.0], vec![0.0, 0.0], vec![0.0, 1.0]];
    assert_eq!(matrix, ans);
}