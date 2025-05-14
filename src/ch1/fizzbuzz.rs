fn main() {
    for row in 0..20 {
        for col in 0..5 {
            let number = row + col * 20 + 1;
            let text = if number % 3 == 0 && number % 5 == 0 {
                "FizzBuzz".to_string()
            } else if number % 3 == 0 {
                "Fizz".to_string()
            } else if number % 5 == 0 {
                "Buzz".to_string()
            } else {
                number.to_string()
            };

            // Print the number or text right-aligned in a 12-character wide field}
            print!("{:>12}", text);
        }
        println!();
    }
}