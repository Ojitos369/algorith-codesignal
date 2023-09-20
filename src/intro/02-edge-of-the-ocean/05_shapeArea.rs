/*
Below we will define an n-interesting polygon. Your task is to find the area of a polygon for a given n.

A 1-interesting polygon is just a square with a side of length 1. An n-interesting polygon is obtained by taking the n - 1-interesting polygon and appending 1-interesting polygons to its rim, side by side. You can see the 1-, 2-, 3- and 4-interesting polygons in the picture below.

https://codesignal.s3.amazonaws.com/uploads/1664318501/area.png

Example

For n = 2, the output should be
solution(n) = 5;
For n = 3, the output should be
solution(n) = 13.
*/

fn solution(d: i32) -> i32 {
    // ((n**2) + ((n-1)**2))
    d.pow(2) + (d-1).pow(2)
}



fn main() {
    let tests = [
        2,
        3
    ];
    let sols = [5, 13];

    for i in 0..tests.len() {
        let d = tests[i].clone();
        let r = solution(d);
        println!("{} -> {} \t {}", sols[i], r, sols[i] == r);
    }
}
