/*
Ratiorg got statues of different sizes as a present from CodeMaster for his birthday, each statue having an non-negative integer size. Since he likes to make things perfect, he wants to arrange them from smallest to largest so that each statue will be bigger than the previous one exactly by 1. He may need some additional statues to be able to accomplish that. Help him figure out the minimum number of additional statues needed.

Example

For statues = [6, 2, 3, 8], the output should be
solution(statues) = 3.

Ratiorg needs statues of sizes 4, 5 and 7.
*/

fn solution(d: Vec<i32>) -> i32 {
    let mut order_d = d.clone();
    order_d.sort();
    let mut needed = 0;
    let len_d = d.len() - 1;
    for i in 0..len_d {
        let diff = order_d[i + 1] - order_d[i];
        if diff > 1 {
            needed += diff - 1;
        }
    }

    needed
}



fn main() {
    // [6, 2, 3, 8]
    // [0, 3]
    let tests: Vec<Vec<i32>> = vec![
        vec![6, 2, 3, 8],
        vec![0, 3],
        vec![1, 2, 3, 8, 9, 5, 4, 7, 4, 5, 6],
    ];
    let sols = [3, 2, 0];

    for i in 0..tests.len() {
        let d = tests[i].clone().to_vec();
        let r = solution(d);
        println!("{} -> {} \t {}", sols[i], r, sols[i] == r);
    }
}
