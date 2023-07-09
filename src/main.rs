/// An implementation of floral formulas
/// following Judd et al., 2002.
use floral::parse_cli;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    match parse_cli() {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }

    Ok(())
}
