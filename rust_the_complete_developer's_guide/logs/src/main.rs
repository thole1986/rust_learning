use std::fs;
use std::io::Error;

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");
    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }
    results
}

fn main() -> Result<(), Error> {

    // Using ? in Rust like try / catch in other languages.
    let text = fs::read_to_string("fdsfsdlogs.txt")?;
    let error_logs = extract_errors(text.as_str());
    fs::write("errors.txt", error_logs.join("\n"))?;

    Ok(())


    // let text = fs::read_to_string("logs.txt")
    //     .expect("failed to read logs.txt");

    // let error_logs = extract_errors(text.as_str());
    // fs::write("errors.txt", error_logs.join("\n"))
    //     .expect("failed to write errors.txt");

    // match fs::read_to_string("logs.txt") {
    //     Ok(text_that_was_read) => {
    //         let error_logs = extract_errors(text_that_was_read.as_str());

    //         match fs::write("errors.txt", error_logs.join("\n")) {
    //             Ok(..) => println!("Wrote errors.txt"),
    //             Err(reason_write_failed) => {
    //                 println!("Writing of errors.txt failed: {}", reason_write_failed)
    //             }
    //         }
    //     }
    //     Err(why_this_failed) => {
    //         println!("Failed to read file: {}", why_this_failed)
    //     }
    // }

    // match divide(5.0, 0.0) {
    //     Ok(result_of_division) => {
    //         println!("{}", result_of_division);
    //     }
    //     Err(what_went_wrong) => {
    //         println!("{}", what_went_wrong);
    //     }
    // }

    // match validate_email(String::from("test@gmail.com")) {
    //     Ok(..) => println!("Email is valid"),
    //     Err(reason_failed) => {
    //         println!("{}", reason_failed)
    //     }
    // }
}

// fn validate_email(email: String) -> Result<(), Error> {
//     if email.contains("@") {
//         // Success
//         Ok(())
//     } else {
//         Err(Error::other("Emails must have an @"))
//     }
// }

// fn divide(a: f64, b: f64) -> Result<f64, Error> {
//     if b == 0.0 {
//         Err(Error::other("Cannot divide by 0"))
//     } else {
//         Ok(a / b)
//     }
// }