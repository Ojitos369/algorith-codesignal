/*
After becoming famous, the CodeBots decided to move into a new building together. Each of the rooms has a different cost, and some of them are free, but there's a rumour that all the free rooms are haunted! Since the CodeBots are quite superstitious, they refuse to stay in any of the free rooms, or any of the rooms below any of the free rooms.

Given matrix, a rectangular matrix of integers, where each value represents the cost of the room, your task is to return the total sum of all rooms that are suitable for the CodeBots (ie: add up all the values that don't appear below a 0).
*/

fn solution(d: Vec<Vec<i32>>) -> i32 {
    println!();
    println!("{:?}", d);
    
    let mut tot = 0;
    let mut cer: Vec<i32> = vec![];
    for i in 0..d.len() {
        for j in 0..d[i].len() {
            if d[i][j] == 0 {
                cer.push(j as i32);
            }
            if !cer.contains(&(j as i32)) {
                tot += d[i][j]
            }
        }
    }
    return tot
}

fn main() {
    let tests = [
        [[0, 1, 1, 2], 
        [0, 5, 0, 0], 
        [2, 0, 3, 3]]
    ];

    let sols = [9];

    for i in 0..tests.len() {
        let d = tests[i].to_vec().iter().map(|x| x.to_vec()).collect();
        let r = solution(d);
        println!("{} -> {} \t {}", sols[i], r, sols[i] == r);
    }
}
