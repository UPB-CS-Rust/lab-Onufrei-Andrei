fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];
    let mut min = 100;
    let mut max = 0;
    let mut x = 0;
    loop {
        if input[x] < min {
            min = input[x];
        }
        if input[x] > max {
            max = input[x];
        }
        if x == input.len() - 1 {
            break;
        }
        x += 1;
    }

    println!("{} is largest and {} is smallest", max, min);
}
