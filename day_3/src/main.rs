use std::collections::HashSet;
fn main() {
    let mut max = 0;
    for line in std::fs::read_to_string("input.txt").unwrap().lines() {

        let half = line.split_at(line.len()/2);
        let common = common_chars(half.0, half.1);
        
        //lowercase
        let mut value = (common as i32)-96;
        if value < 0{
            //uppercase
            value = (common as i32)-38;
        }
        max += value;
    }
    println!("max: {}", max);
}
fn common_chars(s1: &str, s2: &str) -> char {
    let s1: HashSet<_> = s1.chars().collect();
    let s2: HashSet<_> = s2.chars().collect();
    let common: Vec<_> = s1.intersection(&s2).collect();
    *common[0]
 }