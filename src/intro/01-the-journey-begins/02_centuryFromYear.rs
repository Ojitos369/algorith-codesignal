/*
Given a year, return the century it is in. The first century spans from the year 1 up to and including the year 100, the second - from the year 101 up to and including the year 200, etc.

Example

For year = 1905, the output should be
solution(year) = 20;
For year = 1700, the output should be
solution(year) = 17.
*/

fn solution(year: i32) -> i32 {
    println!();
    println!("{:?}", year);
    (year - 1) / 100 + 1
}



fn main() {
    let tests = [
        1905,
        1700
    ];
    let sols = [20, 17];

    for i in 0..tests.len() {
        let d = tests[i].clone();
        let r = solution(d);
        println!("{} -> {} \t {}", sols[i], r, sols[i] == r);
    }
}
