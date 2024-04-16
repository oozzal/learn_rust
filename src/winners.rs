pub fn main() {
    let input = [
        1, 2, 3, 3, 4, 5, 5, 6, 7, 8, 8, 3, 9, 9, 0, 7, 9, 3, 4, 9, 6, 7, 9,
    ];
    let mut winners = vec![];
    winners.push(input[0]);
    for i in 1..input.len() {
        if input[i] > winners[0] {
            winners.clear();
            winners.push(input[i]);
        } else if input[i] == winners[0] {
            winners.push(input[i]);
        }
    }
    println!("{:?}", winners)
}
