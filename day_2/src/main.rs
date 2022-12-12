fn main() {
    let mut max = 0;

    for line in std::fs::read_to_string("input.txt").unwrap().lines() {

        let first = (line.chars().next().unwrap() as i32)-64;
        let last = (line.chars().last().unwrap() as i32)-87;
        max +=  last;

        if first == last{
            max += 3;

        }else if (last-first) == 1 || (last-first) == -2 {
            max += 6;
        }
    }
    println!("max: {}", max);
}
