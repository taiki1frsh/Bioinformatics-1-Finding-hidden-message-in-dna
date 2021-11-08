pub fn hamming_distance(text1: &str, text2: &str)-> u64{
    let mut count = 0;
    //println!("{}", text1);
    for i in 0..text2.len(){
        // print!("{:?}", text2.chars().nth(i).unwrap());
        if text1.chars().nth(i).unwrap() != text2.chars().nth(i).unwrap() {
        count += 1;
        }
    }
    count
}

#[test]
fn test() {
    // sample data
    let geno1 = "GGGCCGTTGGT";
    let geno2 = "GGACCGTTGAC";
    let hamming_distance = hamming_distance(geno1, geno2);

    assert_eq!(hamming_distance, 3);
}