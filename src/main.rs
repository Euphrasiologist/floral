/// An implementation of floral formulas
/// following Judd et al., 2002.
use floral::parse::parse_data;

// the data from our assets folder.
const DATA: &'static str = include_str!("../assets/formulae.csv");

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    parse_data(DATA)?;
    Ok(())
}
