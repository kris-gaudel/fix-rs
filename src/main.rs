mod model;
mod error;

use model::{Field, FixMessage};
use error::FixError;

fn main() {
    println!("FIX Parser - Learning Rust!");
    

    let field = Field::new(35, "D".to_string());
    println!("Field: {:?}", field);

    let fix_message = FixMessage::new();
    println!("FixMessage: {:?}", fix_message);

    let parse_error = FixError::ParseError("Invalid format".to_string());
    let missing_error = FixError::MissingField(35);
    println!("Errors: {:?}, {:?}", parse_error, missing_error);
}
