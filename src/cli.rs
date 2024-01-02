use crate::error::{Error, ErrorKind, Result};
use crate::{
    explain::ExplainFloralFormula,
    floral::{FlowerType, Formula},
};
use std::cmp;

const VERSION: f32 = 0.11;

fn generate_help_str() -> String {
    format!(
        "\
floral v{}

USAGE:
  floral [FLAGS] <TAXON RANK>

FLAGS:
        
  GENERAL FLAGS:
  
  -h, --help            Prints help information
  -a, --all             Print all family information
  -e, --explain         Explain the floral formula
  -v, --version         Print version information only
  -o, --order           Search plant orders, not families
  -d, --data            Return data for plant taxon rank

ARGS:
  <TAXON RANK>          Flowering plant family/order (with -o) name 
",
        VERSION
    )
}

/// Parse the command line arguments, and execute the application.
pub fn parse_args() -> Result<()> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("{}", generate_help_str());
        std::process::exit(0);
    }

    if pargs.contains(["-v", "--version"]) {
        print!("floral v{}", VERSION);
        std::process::exit(0);
    }

    let cli_all = pargs.contains(["-a", "--all"]);
    let cli_explain = pargs.contains(["-e", "--explain"]);
    let cli_order = pargs.contains(["-o", "--order"]);
    let print_data = pargs.contains(["-d", "--data"]);

    let data = crate::parse::parse_data()?;
    let data_keys: Vec<_> = if cli_order {
        data.keys().map(|(o, _, _)| o.to_string()).collect()
    } else {
        data.keys().map(|(_, f, _)| f.to_string()).collect()
    };

    let input_str: Result<String> = match pargs.free_from_str::<String>() {
        Ok(input_s) => Ok(input_s),
        Err(_) => {
            if cli_all {
                Ok("".into()) // don't matter what this string is, it isn't used
            } else {
                print!("{}", generate_help_str());
                std::process::exit(0);
            }
        }
    };

    let rest = pargs.finish();
    let mut input = vec![input_str?];

    let inputs = match rest.is_empty() {
        true => input,
        false => {
            let mut rest: Vec<String> = rest
                .into_iter()
                .map(|e| e.into_string().unwrap_or_default())
                .collect();

            input.append(&mut rest);
            input
        }
    };

    if print_data {
        println!("order\tfamily\tflower_type\tsymmetry\ttepals\tsepals\tpetals\tstamens\tcarpels\tfruit\n");
    }
    for input_str in inputs.clone().into_iter() {
        if let Some((edit_dist, fo_string)) = did_you_mean(&data_keys, &input_str) {
            // so we don't do unexpected things on the cli
            if edit_dist >= 4 && !input_str.is_empty() {
                return Err(Error::new(ErrorKind::GenericCli(format!(
                    "you typed {input_str}, did you mean {fo_string}? Or something else?"
                ))));
            }

            for ((order, family, ft), formula) in data.clone().into_iter() {
                let formatter = DataFormatter::new(
                    cli_all,
                    cli_order,
                    cli_explain,
                    fo_string.clone(),
                    order.to_string(),
                    family.to_string(),
                    ft,
                    formula,
                );
                if print_data {
                    formatter.print_tsv();
                } else {
                    formatter.print();
                }
            }
        }
    }

    Ok(())
}

// gather together all the data we need to print things out properly to the terminal
struct DataFormatter {
    cli_all: bool,
    cli_order: bool,
    cli_explain: bool,
    fo_string: String,
    order: String,
    family: String,
    flower_type: FlowerType,
    formula: Formula,
}

impl DataFormatter {
    #[allow(clippy::too_many_arguments)]
    fn new(
        cli_all: bool,
        cli_order: bool,
        cli_explain: bool,
        fo_string: String,
        order: String,
        family: String,
        flower_type: FlowerType,
        formula: Formula,
    ) -> Self {
        Self {
            cli_all,
            cli_order,
            cli_explain,
            fo_string,
            order,
            family,
            flower_type,
            formula,
        }
    }

    fn print(&self) {
        let format_formula = |order: String,
                              family: String,
                              ft: FlowerType,
                              formula: Formula,
                              explain: bool|
         -> String {
            if explain {
                let explained = formula.explain();
                format!("{order} -> {family}\n{explained}")
            } else {
                format!("{order} -> {family} -> {ft}\n{formula}")
            }
        };
        let family = some_kind_of_uppercase_first_letter(&self.family);
        let formatted = format_formula(
            self.order.clone(),
            family,
            self.flower_type,
            self.formula.clone(),
            self.cli_explain,
        );
        if self.cli_all {
            println!("{}\n", formatted);
        }
        if self.cli_order {
            if self.fo_string == self.order {
                println!("{}\n", formatted);
            }
        } else if self.fo_string == self.family {
            println!("{}\n", formatted);
        }
    }

    fn print_tsv(&self) {
        let mut out = String::new();
        // sort out the variables
        let order = &self.order;
        let family = &self.family;
        let flower_type = self.flower_type;
        let symmetry = self
            .formula
            .get_symmetry()
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let tepals = self.formula.get_tepals().clone().unwrap_or_default();
        let sepals = self.formula.get_sepals().clone().unwrap_or_default();
        let petals = self.formula.get_petals().clone().unwrap_or_default();
        let stamens = self.formula.get_stamens().clone().unwrap_or_default();
        let carpels = self.formula.get_carpels().clone().unwrap_or_default();
        let fruit = self
            .formula
            .get_fruit()
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(",");

        out += &format!("{order}\t{family}\t{flower_type}\t{symmetry}\t{tepals}\t{sepals}\t{petals}\t{stamens}\t{carpels}\t{fruit}");

        if self.cli_all {
            println!("{}", out);
        }
        if self.cli_order {
            if self.fo_string == self.order {
                println!("{}", out);
            }
        } else if self.fo_string == self.family {
            println!("{}", out);
        }
    }
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
