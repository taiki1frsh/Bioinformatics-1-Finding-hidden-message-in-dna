use std::collections::HashMap;

use rand::{distributions::WeightedIndex, prelude::*};

use crate::w3::{profile_kmer::profile_matrix, score::*};

pub fn gibbs_sampler(dna: &str, k: usize, t: usize, mut N: usize) -> Vec<&str> {
    let dna_set: Vec<&str> = dna.split(' ').collect();
    let mut motifs = Vec::new();
    for i in 0..t {
        let ram = rand::thread_rng().gen_range(0..=dna_set[0].len() - k);
        motifs.insert(i, &dna_set[i][ram..ram + k]);
    }

    let mut best_motifs = motifs.clone();
    while N > 0 {
        let i = rand::thread_rng().gen_range(0..t);
        motifs.remove(i);
        let matrix = profile_matrix(motifs.clone());
        motifs.insert(i, profile_random_kmer(dna_set[i], k, matrix));

        if score(motifs.clone()) < score(best_motifs.clone()) {
            best_motifs = motifs.clone();
        }
        N -= 1;
    }
    println!("{}", score(best_motifs.clone()));
    best_motifs
}

// this fn is the key point. In short, this avoids local minima.
fn profile_random_kmer(dna: &str, kmer: usize, profile_matrix: Vec<Vec<f64>>) -> &str {
    let mut map = HashMap::new();
    map.insert('A', 0);
    map.insert('C', 1);
    map.insert('G', 2);
    map.insert('T', 3);

    let mut pmf = Vec::new();
    let mut sum  = 0.0;
    for j in 0..=dna.len() - kmer{
        let kseq = &dna[j..j+kmer];
        let mut score = 1_f64;
        let mut count = 0;
        for k in kseq.chars() {
            score = score * profile_matrix[map[&k]][count];
            count += 1;
        }
        sum += score;
        pmf.insert(j, score);
    }
    for i in 0..pmf.len() {
        pmf[i] = pmf[i] / sum;
    }

    let mut rng = rand::thread_rng();
    let dist = WeightedIndex::new(&pmf).unwrap();
    let motif_num = dist.sample(&mut rng);
    let motif = &dna[motif_num..motif_num + kmer];
    motif
}


#[test]
fn test_gibbs_sampler() {
    let dna = "CGCCCCTCTCGGGGGTGTTCAGTAACCGGCCA GGGCGAGGTATGTGTAAGTGCCAAGGTGCCAG TAGTACCGAGACCGAAAGAAGTATACAGGCGT TAGATCAAGTTTCAGGTGCACGTCGGTGAACC AATCCACCAGCTCCACGTGCAATGTTGGCCTA";
    let k = 8; let t = 5; let N = 100000;
    let gibbs_motifs = gibbs_sampler(dna, k, t, N);

    let a = "TCTCGGGG CCAAGGTG TACAGGCG TTCAGGTG TCCACGTG";
    let ans: Vec<&str> = a.split(' ').collect();
    assert_eq!(gibbs_motifs, ans);
}