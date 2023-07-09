use error::Result;

pub mod error;
pub mod floral;
pub mod parse;

pub fn parse_cli() -> Result<()> {
    let data = parse::parse_data()?;

    for (_, (_, formula)) in data {
        println!("{}", formula);
    }
    Ok(())
}
