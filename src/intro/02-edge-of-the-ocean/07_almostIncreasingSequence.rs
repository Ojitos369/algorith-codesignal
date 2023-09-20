/*
 Given a sequence of integers as an array, determine whether it is possible to obtain a strictly increasing sequence by removing no more than one element from the array.

Note: sequence a0, a1, ..., an is considered to be a strictly increasing if a0 < a1 < ... < an. Sequence containing only one element is also considered to be strictly increasing.

Example

For sequence = [1, 3, 2, 1], the output should be
solution(sequence) = false.

There is no one element in this array that can be removed in order to get a strictly increasing sequence.

For sequence = [1, 3, 2], the output should be
solution(sequence) = true.

You can remove 3 from the array to get the strictly increasing sequence [1, 2]. Alternately, you can remove 2 to get the strictly increasing sequence [1, 3].
*/

fn solution(d: Vec<i32>) -> bool {
    println!();
    println!("{:?}", d);

    if d.len() < 3 {
        return true;
    }
    let mut errores = 0;
    let mut errores_rep = 0;
    for i in 0..d.len() - 1 {
        if d[i] >= d[i+1] {
            errores += 1;
        }
        if i > 0 && d[i-1] >= d[i+1] {
            errores_rep += 1;
        }
        if errores > 1 || errores_rep > 1 {
            return false;
        }
    }
    return true;
}

fn main() {
    let tests: Vec<Vec<i32>> = vec![
        vec![1, 3, 2, 1],
        vec![1, 3, 2],
        vec![3, 6, 5, 8, 10, 20, 15],
        vec![1, 2, 1, 2],
        vec![10, 1, 2, 3, 4, 5],
        vec![0, -2, 5, 6],
        vec![40, 50, 60, 10, 20, 30],
        vec![1, 1],
        vec![1, 2, 3, 4, 3, 6],
        vec![10, 1, 2, 3, 4, 5, 6, 1],
        vec![1, 1, 1, 2, 3],
        vec![1, 1, 1, 2, 3]
    ];

    let sols = [false, true, false, false, true, true, false, true, true, false, false, false];

    for i in 0..tests.len() {
        let d = tests[i].clone();
        let r = solution(d);
        println!("{} -> {} \t {}", sols[i], r, sols[i] == r);
    }
}
