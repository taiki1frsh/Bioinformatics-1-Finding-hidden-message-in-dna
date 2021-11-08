#![feature(test)]

extern crate test;

use test::Bencher;

use practice::w3::median_string::median_string;

#[bench]
fn bench_median_string(b: &mut Bencher) {
    let dnaset = "AAAAGATTTAGAACACCGCCAGCTCCGGTATTAGGACCCCTT CTGCAGGTTAGATCAACAAGCCCAGCAGTTAGGTCGCTATAT ATGTCCGTTCAAGGCACCATTAGACATTTTGACCGCGTACAT ACCATCGCAAAGTGAACATTTAGATAGGACCGAGCTTGATTT CTGCGCCGATTGAGCACAAACCGTCCTACCGCGTGCCTTAGA CTGACGTAAGGTTTTAGACTTGCCTCGTACCAGTCCGATCAG CTTAGAGCACTCTAGTGTGCTAGACGGAACCGTACGTGGGCA CATATTGTCAGACCACGAGTTAGATCAGTTAATTTAAGTGAT CTCCGAGAGGCACAAGTTGTGGGGTTGGGCTTTAGAAAACGA ATTAGAAATAGTCCTAATGGTATTAATCGTGCCGACCGCGGC";
    let dnas: Vec<&str> = dnaset.split(' ').collect();
    b.iter(|| median_string(dnas.clone(), 6))
}
