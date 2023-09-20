/*
Given an array of strings, return another array containing all of its longest strings.

Example

For inputArray = ["aba", "aa", "ad", "vcd", "aba"], the output should be
solution(inputArray) = ["aba", "vcd", "aba"].

*/

fn solution(s1: String, s2: String) -> i32 {
    let mut total = 0;
    let a1: Vec<char> = s1.chars().collect();
    let a2: Vec<char> = s2.chars().collect();
    let mut checked: Vec<char> = vec![];
    for i in 0..a1.len() {
        let char_i = a1[i];
        if checked.contains(&char_i) {
            continue;
        }
        let count_1 = a1.iter().filter(|&x| *x == char_i).count();
        let count_2 = a2.iter().filter(|&x| *x == char_i).count();
        total += std::cmp::min(count_1, count_2) as i32;
        checked.push(char_i);
    }
    total
}

fn main() {
    let tests = [
        ["aabcc", "adcaa"],
        ["zzzz", "zzzzzzz"],
        ["abca", "xyzbac"],
    ];
    let sols = [
        3, 4, 3
    ];

    for i in 0..tests.len() {
        let d = tests[i];
        let r = solution(d[0].to_string(), d[1].to_string());
        println!("{:?} -> {:?} \t {:?}", sols[i], r, sols[i] == r);
    }
}
