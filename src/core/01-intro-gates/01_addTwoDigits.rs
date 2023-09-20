/*
You are given a two-digit integer n. Return the sum of its digits.

Example

For n = 29, the output should be
solution(n) = 11
*/

fn solution(n: i32) -> i32 {
    let mut sum = 0;
    let str_n = n.to_string();
    for c in str_n.chars() {
        sum += c.to_digit(10).unwrap();
    }
    sum as i32
}


fn main() {
    let data = 29;
    let r = solution(data);
    let sol = 11;
    println!("{} -> {} \t {}", sol, r, sol == r);
}
