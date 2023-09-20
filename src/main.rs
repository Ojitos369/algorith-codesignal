/*
Given a year, return the century it is in. The first century spans from the year 1 up to and including the year 100, the second - from the year 101 up to and including the year 200, etc.

Example

For year = 1905, the output should be
solution(year) = 20;
For year = 1700, the output should be
solution(year) = 17.
*/

fn solution(year: i32) -> i32 {
    (year - 1) / 100 + 1
}



fn main() {
    let mut data = 1905;
    let mut r = solution(data);
    let mut sol = 20;
    println!("{} -> {} \t {}", sol, r, sol == r);

    data = 1700;
    r = solution(data);
    sol = 17;
    println!("{} -> {} \t {}", sol, r, sol == r);
}
