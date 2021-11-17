use super::profile_kmer::{median_string_by_probability, profile_matrix};
use super::score::score;

pub fn greedy_motif_search(seq: &str, kmer: usize, t: usize) -> Vec<&str> {
    // initiate values
    let seqs = make_seqs(seq);
    let seq_len = seqs[0].len();
    let mut best_motifs = Vec::new();
    let mut min_score = 10000;

    // main process
    for j in 0..=seq_len - kmer {
        let mut motifs = Vec::new();

        // form motifs by using profile matrix and median string over loop
        motifs.insert(0, &seq[j..j + kmer]);
        for i in 1..t {
            let matrix = profile_matrix(motifs.clone());
            motifs.insert(motifs.len(), median_string_by_probability(seqs[i], kmer, matrix));
        }   

        let score = score(motifs.clone());
        if score < min_score { 
            min_score = score;  
            best_motifs = motifs;
        }
    }        
    best_motifs
}


fn make_seqs(seq: &str) -> Vec<&str> {
    let seqs = seq.split(' ').collect();
    seqs
}


#[test]
fn test_greedy_motif_search() {
    let kmer = 3;
    let t = 5;
    let seq = "GGCGTTCAGGCA AAGAATCAGTCA CAAGGAGTTCGC CACGTCAATCAC CAATAATATTCG";
    let motifs = greedy_motif_search(seq, kmer, t);

    let ans = vec!["CAG", "CAG", "CAA", "CAA", "CAA"];
    assert_eq!(motifs, ans);
}
