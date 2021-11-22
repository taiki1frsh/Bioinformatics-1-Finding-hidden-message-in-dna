use std::collections::BTreeMap;

// as of now pass motif to this fn directly
pub fn score(motifs: Vec<&str>) -> u64 {
    // make a table for calculation but i dont see the solution exp for this.
    let motif_number = &motifs.len();
    let mut motif_table: Vec<Vec<&str>> = vec![vec!["a"; 1]; *motif_number];
    let mut count = 0;
    for i in motifs.clone() {
        motif_table[count][0] = i;
        count += 1;
    }

    // main process
    let motif = search_pos_motif(motif_table.clone());
    let mut score = 0;
    for i in 0..*motif_number {
        for j in 0..motif.len() {
            if motif.chars().nth(j) != motif_table[i][0].chars().nth(j) {
                score += 1;
            }            
        }
    }
    score
}
 
//many to be done
fn search_pos_motif(motif_table: Vec<Vec<&str>>) -> String {
    let mut motif = String::new();
    for i in 0..motif_table[0][0].len() {
        let mut char = BTreeMap::new();
        char.insert('A', 0);
        char.insert('C', 1);
        char.insert('G', 2);
        char.insert('T', 3);
        let mut num = vec![0, 0, 0, 0];

        for j in 0..motif_table.len() {
            let text = motif_table[j][0];
            num[char[&text.chars().nth(i).unwrap()]] += 1;
        }
        let mut max_num_index = 0;
        for i in 0..num.len() {
            if num[max_num_index] < num[i] {
                max_num_index = i;
            }               
        }            
        motif += &char.keys().nth(max_num_index).unwrap().to_string();
     
    }
    motif
}

#[test]
fn test_search_pos_motif() {
    let test = vec![vec!["ATA"], vec!["CTA"], vec!["GTA"], vec!["ATA"]];
    let motif = search_pos_motif(test);

    let ans = "ATA".to_string();
    assert_eq!(motif, ans);
}

#[test]
fn test_score() {
    let table = vec!["ATA", "CTA", "GTA", "ATA"];
    let score = score(table);

    let ans = 2;
    assert_eq!(score, ans);
}