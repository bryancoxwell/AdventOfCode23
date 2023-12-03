use std::fs::read_to_string;

fn main() {
    const INPUT: &str = "../part1_input.txt";
 
    let mut result = 0;
    
    for line in read_to_string(INPUT).unwrap().lines() {
        let ammended = line
            .replace("one", "1")
            .replace("two", "2")
            .replace("three", "3")
            .replace("four", "4")
            .replace("five", "5")
            .replace("six", "6")
            .replace("seven", "7")
            .replace("eight", "8")
            .replace("nine", "9");
        let calibration_values: Vec<_> = ammended.matches(&['1', '2', '3', '4', '5', '6', '7', '8', '9'][..]).collect();
        let first_calibration_value = get_calibration_val(calibration_values[0]);
        let last_calibration_value = get_calibration_val(calibration_values[calibration_values.len() - 1]);

        result += first_calibration_value * 10;
        result += last_calibration_value;
    }
    println!("{}", result);

}

fn get_calibration_val(s: &str) -> i32 {
    match s {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => 0,
    }
}
