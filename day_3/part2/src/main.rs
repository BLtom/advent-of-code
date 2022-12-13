use std::collections::HashSet;
fn main() {
    let mut max = 0;
    let mut count = 0;
    let mut set: HashSet<_> = HashSet::new();
    for line in std::fs::read_to_string("input.txt").unwrap().lines() {
        let line_set: HashSet<_> = line.chars().collect();
 
        if count != 3 {
            if set.is_empty() {
                set = line_set;
            }
            else {
                set = set.intersection(&line_set).map(|c| *c).collect();
            }
        }
        count += 1;
        if count == 3 {
            let elt = set.iter().next().cloned().unwrap();
            let common = set.take(&elt).unwrap();

            let mut value = (common as i32)-96;
            if value < 0{
                value = (common as i32)-38;
            }
            max += value;
            set.clear();
            count = 0;
        }
    }
    println!("max: {}", max);
}