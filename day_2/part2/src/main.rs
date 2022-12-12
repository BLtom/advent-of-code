fn main() {
    let mut max = 0;

    for line in std::fs::read_to_string("input.txt").unwrap().lines() {

        let first = (line.chars().next().unwrap() as i32)-64;
        let last = (line.chars().last().unwrap() as i32)-88;
        max +=  last*3;

        if last == 0{
            let lose_condition = first -1;
            if lose_condition == 0{
                max += 3;
            }else{
                max += lose_condition;
            }
        }else if last == 1{
            max += first;
        }else{
        let win_condition = first +1;
            if win_condition == 4{
                max += 1;
            }else{
                max += win_condition;
            }
        }
    }
    println!("max: {}", max);
}
