use crate::error::Result;
use crate::floral::*;
use std::collections::BTreeMap as Map;
use std::str::FromStr;

// the data from our assets folder.
pub const DATA: &str = include_str!("../assets/formulae.csv");

// function to parse the data into a map
pub fn parse_data<'a>() -> Result<Map<(&'a str, &'a str, FlowerType), Formula>> {
    // skip headers
    let lines = DATA.lines().skip(1);
    let mut data_map = Map::new();
    for line in lines {
        // this is technically a csv parser, but I don't really want the overhead of a
        // fully blown csv parser yet (e.g. csv crate), though it would be nice for errors.
        let line_elements = line.split(',').collect::<Vec<&str>>();

        if let [order, family, flower_type, symmetry, tepals, calyx, petals, anthers, carpels, ovary, fruit, adnation] =
            &line_elements[..]
        {
            let floral = floral_from_str(
                symmetry, tepals, calyx, petals, anthers, carpels, ovary, fruit, adnation,
            )?;
            let ft = FlowerType::from_str(flower_type)?;
            data_map.insert((*order, *family, ft), floral);
        }
    }
    Ok(data_map)
}

// here we do the heavy lifting parsing the csv
#[allow(clippy::too_many_arguments)]
pub fn floral_from_str(
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

    let parsed_sym: Result<Vec<Symmetry>> = sym_vec.iter().map(|e| Symmetry::from_str(e)).collect();
    let parsed_sym = parsed_sym?;

    let parsed_ovary = parse_ovary(ovary)?;

    let parsed_tepals = parse_floral_part_to_enum(tepals, Part::Tepals, None)?;
    let parsed_calyx = parse_floral_part_to_enum(calyx, Part::Calyx, None)?;
    let parsed_petals = parse_floral_part_to_enum(petals, Part::Petals, None)?;
    let parsed_anthers = parse_floral_part_to_enum(anthers, Part::Stamens, None)?;
    let parsed_carpels = parse_floral_part_to_enum(carpels, Part::Carpels, parsed_ovary)?;

    let parsed_adnation = parse_adnation(adnation)?;
    let parsed_fruit = {
        let sp = fruit.split(';').collect::<Vec<&str>>();
        let fruits: Result<Vec<_>> = sp.iter().map(|e| Fruit::from_str(e)).collect();
        fruits?
    };

    let formula = Formula::default()
        .with_symmetry(parsed_sym)
        .with_tepals(parsed_tepals)
        .with_sepals(parsed_calyx)
        .with_petals(parsed_petals)
        .with_stamens(parsed_anthers)
        .with_carpels(parsed_carpels)
        .with_fruit(parsed_fruit)
        .with_adnation(parsed_adnation)
        .build();

    Ok(formula)
}

fn parse_ovary(s: &str) -> Result<Option<Ovary>> {
    if s.is_empty() || s == "-" {
        return Ok(None);
    }

    let sp = s.split(';').collect::<Vec<&str>>();
    let mut ov_vec = Vec::new();

    for el in sp {
        ov_vec.push(Ovary::from_str(el)?);
    }

    if ov_vec.len() > 1 {
        Ok(Some(Ovary::Both))
    } else {
        Ok(Some(ov_vec.clone()[0]))
    }
}

// parse adnation
fn parse_adnation(s: &str) -> Result<Adnation> {
    if s.is_empty() || s == "-" {
        Ok(Adnation::default())
    } else {
        let mut adnation = Adnation::default();
        let sp = s.split(';').collect::<Vec<&str>>();

        for el in sp {
            if el == "v" {
                // it's variable adnation
                adnation.set_variation(true);
            } else {
                // it's a floral part
                let part = Part::from_str(el)?;
                adnation.add_part(part);
            }
        }
        Ok(adnation)
    }
}

// re-used a bunch of times for each of the floral parts.
fn parse_floral_part_to_enum(
    s: &str,
    floral_part: Part,
    ovary: Option<Ovary>,
) -> Result<Option<FloralPart>> {
    if s.is_empty() || s == "-" {
        return Ok(None);
    }

    let sp = s.split(';').collect::<Vec<&str>>();
    let mut floral = FloralPart::default();
    floral.set_ovary(ovary);
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
            // f == fused
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

    Ok(Some(floral))
}
