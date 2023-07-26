use crate::error::Result;
use crate::floral::{
    Adnation, FloralPart, FloralPartNumber, FlowerType, Formula, Fruit, Ovary, Part, Symmetry,
    Whorl,
};
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

    // if anything in the vec of strings contains either
    // an s, c, or a v, we must address this here.
    // mutate the vec at the same time to strip these attributes.
    fn any_contains_vars(s: Vec<&str>) -> (Vec<String>, (bool, bool, bool)) {
        let mut sterile = false;
        let mut connate = false;
        let mut variable = false;

        let mut mutable_string_vec: Vec<String> = s.iter().map(|e| e.to_string()).collect();

        // kind of ugly but works
        for el in mutable_string_vec.iter_mut() {
            sterile = sterile || el.contains('s');
            if sterile {
                *el = el.replace('s', "");
            }
            connate = connate || el.contains('c');
            if connate {
                *el = el.replace('c', "");
            }
            variable = variable || el.contains('v');
            if variable {
                *el = el.replace('v', "");
            }
        }

        (mutable_string_vec, (sterile, connate, variable))
    }

    // e.g. 2-4;f;v
    // this is the 2-4 bit
    for el in sp {
        // we got a range
        if el.contains('-') {
            let split = el.split('-').collect::<Vec<&str>>();

            let (updated_split, (sterile, connate, variable)) = any_contains_vars(split);

            floral.add_whorl(Whorl::new(
                None,
                Some(FloralPartNumber::from_str(&updated_split[0])?),
                Some(FloralPartNumber::from_str(&updated_split[1])?),
                sterile,
                connate,
                variable,
            ));
        } else if el == "c" {
            // c == connate
            floral.set_connation(true);
        } else if el == "v" {
            // v == variable
            floral.set_connation_variation(true);
        } else {
            // it's just a plain number
            let el_single_vec = vec![el];
            let (updated_vec, (sterile, connate, variable)) = any_contains_vars(el_single_vec);

            floral.add_whorl(Whorl::new(
                Some(FloralPartNumber::from_str(&updated_vec[0])?),
                None,
                None,
                sterile,
                connate,
                variable,
            ));
        }
    }

    Ok(Some(floral))
}
