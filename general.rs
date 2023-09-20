/*
 Description of problem
*/

fn solution(d: &str) -> String { // d of data
    println!();
    println!("{:?}", d);
    // ...
    return String::from("correct_answer");
}

fn main() {
    let tests = ["input_data"];
    let sols = ["correct_answer"];

    for i in 0..tests.len() {
        let d = tests[i].clone();
        let r = solution(d);
        println!("{:?} -> {:?} \t {:?}", sols[i], r, sols[i] == r);
    }
}
