/*
Given an array of strings, return another array containing all of its longest strings.

Example

For inputArray = ["aba", "aa", "ad", "vcd", "aba"], the output should be
solution(inputArray) = ["aba", "vcd", "aba"].
*/

fn solution(d: Vec<String>) -> Vec<String> {
    println!();
    println!("{:?}", d);

    let mut max_len = 0;
    // .iter() -> creates an iterator over a slice
    // .fold() -> applies a function to each element of an iterator, passing the result of each such invocation into the next
    // acc -> accumulator
    // s -> current value
    max_len = d.iter().fold(max_len, |acc, s| if s.len() > acc { s.len() } else { acc });
    // d.iter().fold(max_len, |acc, s| if s.len() > acc { s.len() } else { acc });
    // rs = [s for s in d if len(s) == ml]

    let mut rs: Vec<String> = Vec::new();
    rs = d.iter().fold(rs, |acc, s| if s.len() == max_len { acc.push(s.to_string()); acc.to_vec() } else { acc });
    rs

}

fn main() {
    let tests = [
        ["aba", "aa", "ad", "vcd", "aba"]
    ];
    
    let sols = [
        ["aba", "vcd", "aba"]
    ];

    for i in 0..tests.len() {
        let d = tests[i].to_vec();
        let r = solution(d);
        println!("{} -> {} \t {}", sols[i], r, sols[i] == r);
    }
}
