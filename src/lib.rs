use error::Result;

pub mod error;
pub mod floral;
pub mod parse;

pub fn parse_cli() -> Result<()> {
    let data = parse::parse_data()?;

    for (family, (ft, formula)) in data {
        println!("{}", formula);
    }
    Ok(())
}
