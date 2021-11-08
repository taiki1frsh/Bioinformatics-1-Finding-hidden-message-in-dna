#![feature(test)]

extern crate test;

use test::Bencher;

use practice::w3::motif_enumeration::motif_enumeration;

#[bench]
fn bench_motif_enum(b: &mut Bencher) {
    let dnaset = "AGTCCGGTTCGGGGACATAAGTACG TTCACACACACGTCCAGGTATGGAT TTGCGGAATAACCGAGGTCCAGACC CAAATCGCCTAATAGGAAGTTGTCC ATTCCTCAAAAAATGCGTCCTGCCT GTAGGGCTGCGCATCGGTTAGGTCC";
    b.iter(|| motif_enumeration(dnaset, 5, 1));
}