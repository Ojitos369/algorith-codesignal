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

    let tests = [
        [1, 2],
        [3, 4],
        [5, 6],
        [7, 8],
        [9, 0]
    ];
    let sols = [3, 7, 11, 15, 9];

    for i in 0..tests.len() {
        let d = tests[i].clone();
        let r = solution(d[0], d[1]);
        println!("{} -> {} \t {}", sols[i], r, sols[i] == r);
    }
}
