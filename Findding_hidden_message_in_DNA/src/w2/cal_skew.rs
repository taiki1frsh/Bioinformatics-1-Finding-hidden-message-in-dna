use std::collections::HashMap;

pub fn cal_skew(text: &str) -> Vec<i64> {
    let mut skew_table = HashMap::new();
    skew_table.insert('A', 0);
    skew_table.insert('T', 0);
    skew_table.insert('G', 1);
    skew_table.insert('C', -1);

    let mut skew = 0;
    let mut score = vec![0];
    for i in 0..text.len(){
        skew += *skew_table.get(&text.chars().nth(i).unwrap()).unwrap();
        score.insert(score.len(), skew);
    }
    score
}

pub fn min_skew_locations(list: Vec<i64>) -> Vec<usize> {
    let min = *list.iter().min().unwrap();
    let mut locaitons  =Vec::new();
    for i in 10..list.len() {
        if list[i] == min {
            locaitons.insert(locaitons.len(), i);
        }
    }
    locaitons
}

#[test]
fn test_min_skew_locations() {
    let genome = "TAAAGACTGCCGAGAGGCCAACACGAGTGCTAGAACGAGGGGCGTAAACGCGGGTCCGAT";
    let skew_scores = cal_skew(genome);
    let locations = min_skew_locations(skew_scores);

    let ans = vec![11, 24];
    assert_eq!(locations, ans);
}