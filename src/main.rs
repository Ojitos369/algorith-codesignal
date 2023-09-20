/*
Given the string, check if it is a palindrome.

Example

For inputString = "aabaa", the output should be
solution(inputString) = true;
For inputString = "abac", the output should be
solution(inputString) = false;
For inputString = "a", the output should be
solution(inputString) = true.
*/

fn solution(d: String) -> bool {
    // .chars() returns an iterator over the characters of a string slice
    // .rev() reverses the iterator
    // .collect::<String>() collects the iterator into a String
    d == d.chars().rev().collect::<String>()
}



fn main() {
    let tests = [
        String::from("aabaa"),
        String::from("abac"),
        String::from("a")
    ];
    let sols = [true, false, true];

    for i in 0..tests.len() {
        let d = tests[i].clone();
        let r = solution(d);
        println!("{} -> {} \t {}", sols[i], r, sols[i] == r);
    }
}
