// fn main() {
//     for row in 0..9 {
//         for col in 0..9 {
//             // Print the number right-aligned in a 4-character wide field
//             if col == 8 {
//                 // Print the row number at the start of each row
//                 print!("{:>4}", (row + 1) * (col + 1));
//             } else {
//                 // Print the column number at the start of each column
//                 print!("{:>4},", (row + 1) * (col + 1));
//             }
//         }
//         println!();
//     }
// }

// 이중 for 문은 안되.
fn main() {
    for row in 1..10 {
        let s = (1..10)
            .map(|col| format!("{:>4}", row * col))
            .collect::<Vec<_>>()
            .join(",");
        println!("{:>4} | {}", row, s);
    }
}