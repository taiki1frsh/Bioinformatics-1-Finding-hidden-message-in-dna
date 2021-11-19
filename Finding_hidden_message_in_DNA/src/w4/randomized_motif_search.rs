use std::collections::HashMap;
use rand::prelude::*;

use crate::w3::profile_kmer::*;
use crate::w3::score::*;

/* randomized motif search is one of ways to search motifs
the especial feature is the frexible style to determine motifs while the accuracy is 
more certain than median string and greedy motif search.
But, I at least personally at this point don't know how many times is the best to loop.
So, It is trade-off between effiency and accuracy.
*/
pub fn randomized_motif_search(dna: &str, k: usize, t: usize) -> Vec<&str> {
    // prepare necessary variables
    let dna_set: Vec<&str> = dna.split(' ').collect();
    let mut motifs = Vec::new();
    for i in 0..t {
        let ram = rand::thread_rng().gen_range(0..=dna_set[0].len() - k);
        motifs.insert(i, &dna_set[i][ram..ram + k]);
    }

    let mut best_motifs = motifs.clone();
    loop {
        let profile_matrix = profile_matrix(motifs.clone());
        let motifs = make_motifs(profile_matrix, dna_set.clone(), k);
        if score(motifs.clone()) < score(best_motifs.clone()) {
            best_motifs = motifs.clone();
        } else {
            break;
        }
    }
    best_motifs
}

fn make_motifs(profile_matrix: Vec<Vec<f64>>, dna: Vec<&str>, kmer: usize) -> Vec<&str> {
    let mut motifs = Vec::new();
    let mut map = HashMap::new();
    map.insert('A', 0);
    map.insert('C', 1);
    map.insert('G', 2);
    map.insert('T', 3);

    for i in 0..dna.len() {    
        let mut motif = "";    
        let mut max_score = 0_f64;

        for j in 0..=dna[0].len() - kmer{
            let kseq = &dna[i][j..j+kmer];
            let mut score = 1_f64;
            let mut count = 0;
            for k in kseq.chars() {
                score = score * profile_matrix[map[&k]][count];
                count += 1;
            }
            
            if score >= max_score {
                max_score = score;
                motif = kseq;
            }
        }
        motifs.insert(i, motif);
    }
    motifs
}

#[test]
fn test_make_motifs() {
    let profile = vec![
        vec![5.0/ 9.0, 1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0],
        vec![1.0 / 9.0, 4.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0],
        vec![1.0 / 9.0, 1.0 / 9.0, 5.0 / 9.0, 1.0 / 9.0],
        vec![1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0, 5.0/ 9.0],
    ];
    let dna = vec![
        "TTACCTTAAC",
        "GATGTCTGTC",
        "ACGGCGTTAG",
        "CCCTAACGAG",
        "CGTCAGAGGT"];
    let ans = vec!["ACCT", "ATGT", "GCGT", "ACGA", "AGGT"];
    let motifs = make_motifs(profile, dna, 4);

    assert_eq!(motifs, ans);
}

#[test]
fn test_randomized_motif_search() {
    let k = 8; let t = 5;
    let dna = "CGCCCCTCTCGGGGGTGTTCAGTAAACGGCCA GGGCGAGGTATGTGTAAGTGCCAAGGTGCCAG TAGTACCGAGACCGAAAGAAGTATACAGGCGT TAGATCAAGTTTCAGGTGCACGTCGGTGAACC AATCCACCAGCTCCACGTGCAATGTTGGCCTA";
    let mut n = 0;
    let mut best_score = 10000;
    let mut final_motifs = Vec::new();

    while n < 1001 {
        let motifs = randomized_motif_search(dna, k, t);
        let cur_score = score(motifs.clone());
        if cur_score < best_score {
            best_score = cur_score;
            final_motifs = motifs;
        }
        n += 1;
    }

    let ans = vec!["TCTCGGGG", "CCAAGGTG", "TACAGGCG", "TTCAGGTG", "TCCACGTG"];
    assert_eq!(final_motifs, ans);
}