fn main() {
        let mut sum = 0;
        let mut max = 0;
        for line in std::fs::read_to_string("input.txt").unwrap().lines() {
            if !line.is_empty() {
                sum += line.parse::<i32>().unwrap();
            }
            else {
                if sum > max {
                    max = sum;
                }
                sum = 0;
            }
        }
        println!("max: {}", max);
}
