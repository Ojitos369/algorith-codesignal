/*
 Description of problem
*/

fn solution(d: &str) -> String {
    println!();
    println!("{}", d);
    // ...
    return String::from("correct_answer");
}

fn main() {
    let data = "input_data";
    let r = solution(data);
    let sol = "correct_answer";
    println!("{} -> {} \t {}", sol, r, sol == r);
}
