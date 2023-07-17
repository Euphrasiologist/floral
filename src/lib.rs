use crate::floral::{FlowerType, Formula};
use error::Result;
use std::cmp;

pub mod error;
pub mod floral;
pub mod parse;

const HELP: &str = "\
floral

USAGE:
  floral [FLAGS] <STRING>

FLAGS:
  -h, --help            Prints help information
  -a, --all             Print all family information
  -e, --explain         Explain the floral formula
  -c, --compact         Print compact output
  -o, --order           Search plant orders, not families

ARGS:
  <STRING>              Flowering plant order/family name 
";

pub fn parse_args() -> Result<()> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let cli_all = pargs.contains(["-a", "--all"]);
    let cli_explain = pargs.contains(["-e", "--explain"]);
    let cli_compact = pargs.contains(["-c", "--compact"]);
    let cli_order = pargs.contains(["-o", "--order"]);

    let data = parse::parse_data()?;
    let data_keys: Vec<_> = if cli_order {
        data.keys().map(|(o, _, _)| o.to_string()).collect()
    } else {
        data.keys().map(|(_, f, _)| f.to_string()).collect()
    };

    let input_str = match pargs.free_from_str::<String>() {
        Ok(input_s) => Ok(input_s),
        Err(err) => {
            if cli_all {
                Ok("".into()) // don't matter what this string is, it isn't used
            } else {
                Err(err)
            }
        }
    };

    let input_str = input_str?;

    if let Some((edit_dist, fo_string)) = did_you_mean(&data_keys, &input_str) {
        // so we don't do unexpected things on the cli
        if edit_dist >= 4 && !input_str.is_empty() {
            eprintln!("You typed {input_str}, did you mean {fo_string}? Or something else?");
            std::process::exit(0);
        }

        let format_formula =
            |order: &str, family: String, ft: FlowerType, formula: Formula| -> String {
                format!("{order} -> {family} -> {ft}\n{formula}")
            };

        for ((order, family, ft), formula) in data {
            if cli_all {
                let family = some_kind_of_uppercase_first_letter(family);
                let formatted = format_formula(order, family, ft, formula);
                println!("{}\n", formatted);
                continue;
            }
            if cli_order {
                if fo_string == order {
                    let family = some_kind_of_uppercase_first_letter(family);
                    let formatted = format_formula(order, family, ft, formula);
                    println!("{}\n", formatted);
                }
            } else if fo_string == family {
                let family = some_kind_of_uppercase_first_letter(family);
                let formatted = format_formula(order, family, ft, formula);
                println!("{}\n", formatted);
            }
        }
    }

    Ok(())
}

fn did_you_mean(possibilities: &[String], tried: &str) -> Option<(usize, String)> {
    let mut possible_matches: Vec<_> = possibilities
        .iter()
        .map(|word| {
            let edit_distance = levenshtein_distance(&word.to_lowercase(), &tried.to_lowercase());
            (edit_distance, word.to_owned())
        })
        .collect();

    possible_matches.sort();

    if let Some((edit, first)) = possible_matches.into_iter().next() {
        Some((edit, first))
    } else {
        None
    }
}

fn levenshtein_distance(a: &str, b: &str) -> usize {
    lev_distance(a, b, usize::max_value()).unwrap_or(usize::max_value())
}

// see https://github.com/nushell/nushell/blob/99329f14a3db6945771725d65b1b553563ce6b28/crates/nu-protocol/src/lev_distance.rs#L57
fn lev_distance(a: &str, b: &str, limit: usize) -> Option<usize> {
    let n = a.chars().count();
    let m = b.chars().count();
    let min_dist = if n < m { m - n } else { n - m };

    if min_dist > limit {
        return None;
    }
    if n == 0 || m == 0 {
        return Some(min_dist);
    }

    let mut dcol: Vec<_> = (0..=m).collect();

    for (i, sc) in a.chars().enumerate() {
        let mut current = i;
        dcol[0] = current + 1;

        for (j, tc) in b.chars().enumerate() {
            let next = dcol[j + 1];
            if sc == tc {
                dcol[j + 1] = current;
            } else {
                dcol[j + 1] = cmp::min(current, next);
                dcol[j + 1] = cmp::min(dcol[j + 1], dcol[j]) + 1;
            }
            current = next;
        }
    }

    if dcol[m] <= limit {
        Some(dcol[m])
    } else {
        None
    }
}

fn some_kind_of_uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
