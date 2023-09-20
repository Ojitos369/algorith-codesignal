/*
Ratiorg got statues of different sizes as a present from CodeMaster for his birthday, each statue having an non-negative integer size. Since he likes to make things perfect, he wants to arrange them from smallest to largest so that each statue will be bigger than the previous one exactly by 1. He may need some additional statues to be able to accomplish that. Help him figure out the minimum number of additional statues needed.

Example

For statues = [6, 2, 3, 8], the output should be
solution(statues) = 3.

Ratiorg needs statues of sizes 4, 5 and 7.
*/

fn solution(mut d: Vec<i32>) -> i32 {
    println!();
    println!("{:?}", d);

    // .sort() -> orders the vector in ascending order
    d.sort();

    // .windows(2) -> returns a sliding window iterator over the vector
    // .map() -> returns an iterator that applies the given closure to each element
    // .sum() -> returns the sum of all elements in the iterator
    d.windows(2).map(|pair| pair[1] - pair[0] - 1).sum()

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
