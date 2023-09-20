/*
Given an array of strings, return another array containing all of its longest strings.

Example

For inputArray = ["aba", "aa", "ad", "vcd", "aba"], the output should be
solution(inputArray) = ["aba", "vcd", "aba"].

*/

fn solution(s1: String, s2: String) -> i32 {

}

fn main() {
    let tests = [
        ["aba", "aa", "ad", "vcd", "aba"]
    ];
    
    let sols = [
        ["aba", "vcd", "aba"]
    ];

    for i in 0..tests.len() {
        let d = tests[i].iter().map(|s| s.to_string()).collect();
        let r = solution(d);
        let sr: Vec<String> = sols[i].iter().map(|s| s.to_string()).collect();
        println!("{:?} -> {:?} \t {:?}", sr, r, sr == r);
    }
}
