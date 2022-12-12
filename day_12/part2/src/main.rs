fn main() {
    let mut sum = 0;
        let mut max_three = [0, 0, 0];
        for line in std::fs::read_to_string("input.txt").unwrap().lines() {
            if !line.is_empty() {
                sum += line.parse::<i32>().unwrap();
            }
            else {
                if sum > max_three[0] {
                    max_three[2] = max_three[1];
                    max_three[1] = max_three[0];
                    max_three[0] = sum;
                }else if sum > max_three[1] {
                    max_three[2] = max_three[1];
                    max_three[1] = sum;
                } else if sum > max_three[2] {
                    max_three[2] = sum;
                }
                sum = 0;
            }
        }
        let result: i32 = max_three.iter().sum();
        println!("result: {}", result);
}
