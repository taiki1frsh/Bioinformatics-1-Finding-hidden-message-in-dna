// use a recursive way
pub fn neighbors(pattern:&mut String, permissible_distance: u64) -> Vec<String> {
    let mut neighborhood = Vec::new();
    let bases = vec!["A".to_string(), "T".to_string(), "G".to_string(), "C".to_string()];

    if permissible_distance == 0 {
        neighborhood.insert(0,pattern.clone());
        println!("perm_dis == 0 : {:?}", neighborhood);
        neighborhood
    } else if pattern.len() == 1 {
        bases
    } else {
        let suffix_neighbors = neighbors(&mut suffix(pattern), permissible_distance);
        for  text in suffix_neighbors{
            if nei_hamming_distance(pattern, &text) < permissible_distance {
                for x in bases.clone() {
                    let new_tex = x + &text;
                    neighborhood.insert(neighborhood.len(), new_tex);
                }
            } else {
                let new_tex = pattern.chars().nth(0).unwrap().to_string() + &text;
                neighborhood.insert(neighborhood.len(), new_tex);
            }
        }
        neighborhood
    }
}

// to extract the suffix part of the arguememted string
fn suffix(text: &str) -> String {
    let mut mut_tx = text.to_string();
    let _s = mut_tx.remove(0);
    mut_tx
}

fn nei_hamming_distance(text1: &str, text2: &str)-> u64{
    let mut count = 0;
    //println!("{}", text1);
    for i in 0..text2.len(){
        // print!("{:?}", text2.chars().nth(i).unwrap());
        if text1.chars().nth(i + 1).unwrap() != text2.chars().nth(i).unwrap() {
        count += 1;
        }
    }
    count
}

#[test]
fn test_neighbors() {
    let mut pattern = "ACG".to_string();
    let permissible_dis= 1;
    let mut immediate_neighbors = neighbors(&mut pattern, permissible_dis);
    immediate_neighbors.sort_unstable();

    let mut ans = vec!["CCG", "TCG", "GCG", "AAG", "ATG", "AGG", "ACA", "ACC", "ACT", "ACG"];
    ans.sort_unstable();
    assert_eq!(immediate_neighbors, ans);
}

#[test]
fn test_suffix() {
    let mut text = "hello".to_string();
    let suff = suffix(&mut text);

    let ans = "ello";
    assert_eq!(suff, ans);
}