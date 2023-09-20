/*
Given an array of integers, find the pair of adjacent elements that has the largest product and return that product.

Example

For inputArray = [3, 6, -2, -5, 7, 3], the output should be
solution(inputArray) = 21.

7 and 3 produce the largest product.
*/

fn solution(d: Vec<i32>) -> i32 {
    println!();
    println!("{:?}", d);

    if d.len() > 2 {
        return std::cmp::max(d[0]*d[1], solution(d[1..].to_vec()));
    } else {
        return d[0]*d[1];
    }
}



fn main() {
    let tests = [
        [3, 6, -2, -5, 7, 3],
    ];
    let sols = [21];

    for i in 0..tests.len() {
        let d = tests[i].clone().to_vec();
        let r = solution(d);
        println!("{} -> {} \t {}", sols[i], r, sols[i] == r);
    }
}
