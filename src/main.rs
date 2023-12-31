
use std::io;
use std::io::Write;
// Using a Test Driven Development, create a program that calculates the area of a room. Prompt
// the user for the length and width of the room in feet. Then
// display the area in both square feet and square meters.

// A program that calculates the area of a room in square feet, and square meters by given length and width in feet
// Inputs: length and width in feet
// Process: Calculates the area in square feet, convert to square meters
// Output: Display the area in both square feet and square meters

fn round_decimal(number: f64) -> f64 {
    (number * 1000.0).round() / 1000.0
}

fn calculate_area_feet(length: f64, width: f64) -> f64 {
    length * width
}

fn convert_to_square_meters(area_feet: f64) -> f64 {
    const SQFT_TO_SQM: f64 = 0.09290304;
    round_decimal(area_feet * SQFT_TO_SQM)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_decimal() {
        assert_eq!(round_decimal(27.870912), 27.871);
        assert_eq!(round_decimal(2.3225), 2.323);
    }

    #[test]
    fn test_calculate_area_feet() {
        assert_eq!(calculate_area_feet(15.0, 20.0), 300.0);
        assert_eq!(calculate_area_feet(15.0, 20.0), 300.0);
        assert_eq!(calculate_area_feet(5.0, 5.0), 25.0);
    }

    #[test]
    fn test_convert_to_square_meters() {
        assert_eq!(convert_to_square_meters(300.0), 27.871);
        assert_eq!(convert_to_square_meters(25.0), 2.323);
    }

}


fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn main() {
    let length: f64 = get_input("What is the length of the room in feet? ");
    let width: f64 = get_input("What is the width of the room in feet? ");

    let area_sqft: f64 = calculate_area_feet(length, width);
    let area_sqm: f64 = convert_to_square_meters(area_sqft);

    println!("You entered dimensions of {} feet by {} feet.", length, width);
    println!("The area is");
    println!("{} square feet", area_sqft);
    println!("{} square meters", area_sqm);

}
