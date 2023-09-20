/*
Write a function that returns the sum of two numbers.

Example

For param1 = 1 and param2 = 2, the output should be
solution(param1, param2) = 3.
*/

fn solution(param1: i32, param2: i32) -> i32 {
    param1 + param2
}


fn main() {
    let data = [1, 2];
    let r = solution(data[0], data[1]);
    let sol = 3;
    println!("{} -> {} \t {}", sol, r, sol == r);
}
