use std::collections::HashMap;

pub fn rev_complements(genome: &str) -> String {
    let length = genome.len();
    let mut comple_table = HashMap::new();
    comple_table.insert('A', "T");
    comple_table.insert('T', "A");
    comple_table.insert('G', "C");
    comple_table.insert('C', "G");
    let rev_genome: String = genome.chars().rev().collect();
    let mut rev_comp_genome = String::new();

    for i in 0..length {
        rev_comp_genome += comple_table[&rev_genome.chars().nth(i).unwrap()];
    }
    
    rev_comp_genome
}

#[test]
fn test_rev_complements() {
    // sample data
    let genome = "ATGCCGTA";
    let rev_comp_genome = rev_complements(genome);

    assert_eq!(rev_comp_genome, "TACGGCAT");
}