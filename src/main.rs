/*
Given an array of strings, return another array containing all of its longest strings.

Example

For inputArray = ["aba", "aa", "ad", "vcd", "aba"], the output should be
solution(inputArray) = ["aba", "vcd", "aba"].

*/

fn solution(d: Vec<String>) -> Vec<String> {
    println!();
    println!("{:?}", d);

    // .iter() -> iterator over references
    // .map() -> transform each element
    // .max() -> find the max element
    // .unwrap() -> get the value out of the Option
    let max_len = d.iter().map(|s| s.len()).max().unwrap();

    // .into_iter() -> iterator over values
    // .filter() -> filter the values
    // .collect() -> collect the values into a vector
    d.into_iter().filter(|s| s.len() == max_len).collect()
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
