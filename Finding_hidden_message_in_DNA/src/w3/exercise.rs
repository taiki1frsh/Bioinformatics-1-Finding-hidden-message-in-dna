// for exercise break
fn kmer_expectation(genome_size: u64, kmer: u64) -> f64 {
    all_pair_size = (genome_size - kmer) * kmer;
    println!("{}", all_pair_size);
    expectation = (1 / 4) ** kmer * (all_pair_size);
    expectation
}

#[test]
fn test_kmer_expectation() {
    let exp = kmer_expectation(500000, 9);
    println!("exp:  {}", exp);
    println!("hello world");
}