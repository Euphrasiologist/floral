use crate::error::Result;
use crate::floral::*;
use std::collections::HashMap;
use std::str::FromStr;

// the data from our assets folder.
const DATA: &'static str = include_str!("../assets/formulae.csv");

// function to parse the data into a map
pub fn parse_data(data: &'static str) -> Result<()> {
    let lines = data.lines().skip(1);
    for line in lines {
        // this is technically a csv parser, but I don't really want the overhead of a
        // fully blown csv parser yet (e.g. csv crate)
        let line_elements = line.split(',').collect::<Vec<&str>>();

        let mut data_map = HashMap::new();

        if let [order, family, flower_type, symmetry, tepals, calyx, petals, anthers, carpels, ovary, fruit, adnation] =
            &line_elements[..]
        {
            let floral = floral_from_str(
                symmetry, tepals, calyx, petals, anthers, carpels, ovary, fruit, adnation,
            )?;
            let ft = FlowerType::from_str(flower_type)?;
            println!("{family} {ft} {floral}");
            data_map.insert(family, (ft, floral));
        }
    }
    Ok(())
}

// here we do the heavy lifting parsing the csv
fn floral_from_str(
    symmetry: &str,
    tepals: &str,
    calyx: &str,
    petals: &str,
    anthers: &str,
    carpels: &str,
    ovary: &str,
    fruit: &str,
    adnation: &str,
) -> Result<Formula> {
    let sym_vec = symmetry.split(';').collect::<Vec<&str>>();

    let parsed_sym = sym_vec
        .iter()
        // deal with this unwrap later
        .map(|e| Symmetry::from_str(e).unwrap())
        .collect::<Vec<Symmetry>>();

    let parsed_tepals = parse_floral_part_to_enum(tepals, Part::Tepals)?;
    let parsed_calyx = parse_floral_part_to_enum(calyx, Part::Calyx)?;
    let parsed_petals = parse_floral_part_to_enum(petals, Part::Petals)?;
    let parsed_anthers = parse_floral_part_to_enum(anthers, Part::Stamens)?;
    let parsed_carpels = parse_floral_part_to_enum(carpels, Part::Carpels)?;

    // parse_ovary/parse_fruit/parse_adnation
    Ok(Formula::new(
        parsed_sym,
        parsed_tepals,
        parsed_calyx,
        parsed_petals,
        parsed_anthers,
        parsed_carpels,
        None,
        None,
        None,
    ))
}

fn parse_floral_part_to_enum(s: &str, floral_part: Part) -> Result<Option<FloralPart>> {
    if s == "" || s == "-" {
        return Ok(None);
    }

    let sp = s.split(';').collect::<Vec<&str>>();
    let mut floral = FloralPart::default();
    floral.set_part(floral_part);

    // e.g. 2-4;f;v
    // this is the 2-4 bit
    for el in sp {
        // we got a range
        if el.contains('-') {
            let split = el.split('-').collect::<Vec<&str>>();

            if split[0].contains('s') || split[1].contains('s') {
                // this means sterile floral part
                // parse the left
                let min = FloralPartNumber::from_str(&split[0].replace('s', ""))?;
                let max = FloralPartNumber::from_str(&split[1].replace('s', ""))?;

                floral.add_whorl(Whorl::new(None, Some(min), Some(max), Sterile::Sterile));
            } else {
                let min = FloralPartNumber::from_str(split[0])?;
                let max = FloralPartNumber::from_str(split[1])?;

                floral.add_whorl(Whorl::new(None, Some(min), Some(max), Sterile::Fertile));
            }
        } else if el == "f" {
            floral.set_connation(true);
            // deal with f/v, and numbers last.
            // we got a number
        } else if el == "v" {
            floral.set_connation_variation(true);
        } else {
            // it's just a plain number
            if el.contains('s') {
                // it's sterile
                floral.add_whorl(Whorl::new(
                    Some(FloralPartNumber::from_str(&el.replace('s', ""))?),
                    None,
                    None,
                    Sterile::Sterile,
                ));
            } else {
                // it's fertile!
                floral.add_whorl(Whorl::new(
                    Some(FloralPartNumber::from_str(el)?),
                    None,
                    None,
                    Sterile::Fertile,
                ));
            }
        }
    }
    // the relevant fields will stay as None

    Ok(Some(floral))
}
