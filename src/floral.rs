use std::fmt::{self, Display};
use std::result;
use std::str::FromStr;

use crate::error::{Error, ErrorKind};

/// If the user wants an explanation of the floral parts
trait ExplainFloralFormula {
    fn explain(&self) -> String;
}

/// The type of flower we're looking at
pub enum FlowerType {
    Bisexual,
    Carpellate,
    Staminate,
}

impl ExplainFloralFormula for FlowerType {
    fn explain(&self) -> String {
        let flower_type = match self {
            FlowerType::Bisexual => "bisexual",
            FlowerType::Carpellate => "carpellate",
            FlowerType::Staminate => "staminate",
        };

        format!("A {} flower", flower_type)
    }
}

impl Display for FlowerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlowerType::Bisexual => write!(f, "Bisexual"),
            FlowerType::Carpellate => write!(f, "Carpellate"),
            FlowerType::Staminate => write!(f, "Staminate"),
        }
    }
}

impl FromStr for FlowerType {
    type Err = Error;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        match s {
            "b" => Ok(Self::Bisexual),
            "c" => Ok(Self::Carpellate),
            "s" => Ok(Self::Staminate),
            input_str => Err(Error::new(ErrorKind::FromStr(format!(
                "Input string: \"{}\" not recognised.",
                input_str
            )))),
        }
    }
}

/// The floral symmetry
#[derive(Debug)]
pub enum Symmetry {
    /// Infinitely many symmetries
    Radial,
    /// One line of symmetry
    /// We encase a BilateralType as there are
    /// different kinds of bilateral symmetry
    Bilateral(BilateralType),
    /// No lines of symmetry
    Asymmetry,
    /// Spiral
    Spiral,
    /// Disymmetric
    Disymmetric,
}

#[derive(Debug)]
pub enum BilateralType {
    Up,
    Down,
    Left,
    Right,
    Upleft,
    Upright,
    Downleft,
    Downright,
}

impl Display for BilateralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BilateralType::Up => write!(f, "↑"),
            BilateralType::Down => write!(f, "↓"),
            BilateralType::Left => write!(f, "←"),
            BilateralType::Right => write!(f, "→"),
            BilateralType::Upleft => write!(f, "↖"),
            BilateralType::Upright => write!(f, "↗"),
            BilateralType::Downleft => write!(f, "↙"),
            BilateralType::Downright => write!(f, "↘"),
        }
    }
}

impl FromStr for BilateralType {
    type Err = Error;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        match s {
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            "left" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            "upleft" => Ok(Self::Upleft),
            "upright" => Ok(Self::Upright),
            "downleft" => Ok(Self::Downleft),
            "downright" => Ok(Self::Downright),
            err => Err(Error::new(ErrorKind::FromStr(format!(
                "Error converting string to BilateralType: {}",
                err
            )))),
        }
    }
}

impl Display for Symmetry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symmetry::Radial => write!(f, "*"),
            Symmetry::Bilateral(b) => write!(f, "X({})", b),
            Symmetry::Asymmetry => write!(f, "↯"),
            Symmetry::Spiral => write!(f, "↻"),
            Symmetry::Disymmetric => write!(f, "↔"),
        }
    }
}

impl FromStr for Symmetry {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "r" => Ok(Self::Radial),
            "up" => Ok(Self::Bilateral(BilateralType::Up)),
            "down" => Ok(Self::Bilateral(BilateralType::Down)),
            "left" => Ok(Self::Bilateral(BilateralType::Left)),
            "right" => Ok(Self::Bilateral(BilateralType::Right)),
            "upleft" => Ok(Self::Bilateral(BilateralType::Upleft)),
            "upright" => Ok(Self::Bilateral(BilateralType::Upright)),
            "downleft" => Ok(Self::Bilateral(BilateralType::Downleft)),
            "downright" => Ok(Self::Bilateral(BilateralType::Downright)),
            "a" => Ok(Self::Asymmetry),
            "s" => Ok(Self::Spiral),
            "d" => Ok(Self::Disymmetric),
            _ => Err(Error::new(ErrorKind::FromStr(
                "Some other error in parsing the symmetry".into(),
            ))),
        }
    }
}

/// The number of parts in a floral organ.
/// Infinity, is something like > 30.
#[derive(Debug)]
pub enum FloralPartNumber {
    /// A finite value
    Finite(u32),
    /// A value > 30.
    Infinite,
}

impl FromStr for FloralPartNumber {
    type Err = Error;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        if s == "-" {
            return Ok(FloralPartNumber::Finite(0));
        }
        if s == "inf" {
            return Ok(FloralPartNumber::Infinite);
        }

        let num = match s.parse::<u32>() {
            Ok(n) => n,
            Err(err) => return Err(Error::new(ErrorKind::ParseInt(err))),
        };

        // TODO: is this right?
        match num {
            0..=30 => Ok(FloralPartNumber::Finite(num)),
            31.. => Ok(FloralPartNumber::Infinite),
        }
    }
}

impl Display for FloralPartNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FloralPartNumber::Finite(u) => write!(f, "{}", u),
            FloralPartNumber::Infinite => write!(f, "∞"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Adnation {
    variation: bool,
    parts: Option<Vec<Part>>,
}

impl Adnation {
    pub fn set_variation(&mut self, variation: bool) {
        self.variation = variation;
    }
    pub fn add_part(&mut self, part: Part) {
        if self.parts.is_none() {
            self.parts = Some(vec![]);
        }
        if let Some(e) = self.parts.as_mut() {
            e.push(part)
        }
    }
}

/// The total floral formula
#[derive(Debug)]
pub struct Formula {
    /// Floral symmetry
    symmetry: Vec<Symmetry>,
    /// Tepals
    tepals: Option<FloralPart>,
    /// Sepals
    sepals: Option<FloralPart>,
    /// Petals
    petals: Option<FloralPart>,
    /// Stamens
    stamens: Option<FloralPart>,
    /// Carpels
    carpels: Option<FloralPart>,
    /// Ovary
    ovary: Option<Ovary>,
    /// Fruit
    fruit: Vec<Fruit>,
    /// Where is the adnation present?
    adnation: Adnation,
}

impl Formula {
    pub fn new(
        symmetry: Vec<Symmetry>,
        tepals: Option<FloralPart>,
        sepals: Option<FloralPart>,
        petals: Option<FloralPart>,
        stamens: Option<FloralPart>,
        carpels: Option<FloralPart>,
        ovary: Option<Ovary>,
        fruit: Vec<Fruit>,
        adnation: Adnation,
    ) -> Self {
        Self {
            symmetry,
            tepals,
            sepals,
            petals,
            stamens,
            carpels,
            ovary,
            fruit,
            adnation,
        }
    }
}

#[derive(Debug, Default)]
struct AdnationIndex {
    tepals: Option<usize>,
    sepals: Option<usize>,
    petals: Option<usize>,
    stamens: Option<usize>,
    carpels: Option<usize>,
}

// this is the trait which will display the adnation
// between floral parts as table unicode chars
impl Display for AdnationIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // merge into a vec
        let merged = vec![
            self.tepals,
            self.sepals,
            self.petals,
            self.stamens,
            self.carpels,
        ];
        // TODO: these should be ordered already... but maybe check this?
        let merged_only_some: Vec<_> = merged.into_iter().flatten().collect();

        match merged_only_some.len() {
            0 | 1 => write!(f, ""), // nothing to do
            2 => {
                // link between two elements
                let mut adnation = String::new();
                for _ in 0..merged_only_some[0] {
                    adnation.push(' ');
                }
                adnation.push('╰');
                for _ in merged_only_some[0]..merged_only_some[1] - 1 {
                    adnation.push('─');
                }
                adnation.push('╯');
                write!(f, "{}", adnation)
            }
            fusions @ 3.. => {
                // three or more adnations, fancy
                let mut adnation = String::new();
                let mut adnation_iter = (0..fusions).peekable();

                while let Some(fusion) = adnation_iter.next() {
                    // if we are on the first iteration
                    if fusion == 0 {
                        // we want spaces up until this point.
                        for _ in 0..merged_only_some[fusion] {
                            adnation.push(' ');
                        }
                        adnation.push('╰');
                        // and continue to next loop iteration
                        continue;
                    }

                    // dashes in between
                    for _ in merged_only_some[fusion - 1]..merged_only_some[fusion] - 1 {
                        adnation.push('─');
                    }

                    // and we close on the last iteration
                    if adnation_iter.peek().is_none() {
                        adnation.push('╯');
                        // just to be sure.
                        break;
                    }
                    // and if we are in between iterations,
                    // we want this funky char
                    adnation.push('┴');
                }
                write!(f, "{}", adnation)
            }
            _ => return Err(fmt::Error),
        }
    }
}

// TODO: remove the println!() statements
impl Display for Formula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sym = &self
            .symmetry
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(" or ");

        // we start the index at wherever the symmetry ends
        // plus one for comma in first element
        let mut format_index = sym.chars().count() + 1;
        println!("Format index at start: {}", format_index);

        let adnation_vec = self.adnation.parts.clone().unwrap_or(vec![]);
        let mut adnation_status = AdnationIndex::default();

        let calyx_perianth_or_tepals = if let Some(t) = &self.tepals {
            // calyx and petals not needed
            // probably make this into a proper error.
            assert!(self.sepals.is_none() && self.petals.is_none());
            // check if tepals are in the adnation vec.
            if adnation_vec.contains(&Part::Tepals) {
                adnation_status.tepals = Some(format_index);
            }
            println!("Format index at tepals: {}", format_index);
            // the tepal string to return
            let tepal_string = format!(",{}", t);
            println!("tepal string length: {}", tepal_string.chars().count());
            // increment the format index
            format_index += tepal_string.chars().count();
            tepal_string
        } else {
            // these unwraps are safe.
            let calyx = self.sepals.as_ref().unwrap().to_string();
            let petals = self.petals.as_ref().unwrap().to_string();
            // make calyx string here
            let calyx_string = format!(",{}", calyx);
            let petals_string = format!(",{}", petals);

            // deal with adnation logic here
            if adnation_vec.contains(&Part::Calyx) {
                adnation_status.sepals = Some(format_index);
            }
            println!("Format index at sepals: {}", format_index);
            println!("Sepal string length: {}", calyx_string.chars().count());
            // increment the format index
            format_index += calyx_string.chars().count();
            if adnation_vec.contains(&Part::Petals) {
                adnation_status.petals = Some(format_index);
            }
            println!("Format index at petals: {}", format_index);
            println!("Petal string length: {}", petals_string.chars().count());
            // increment the format index again
            format_index += petals_string.chars().count();
            format!("{}{}", calyx_string, petals_string)
        };

        let anthers = if let Some(a) = &self.stamens {
            let anthers_string = format!(",{}", a);

            if adnation_vec.contains(&Part::Stamens) {
                adnation_status.stamens = Some(format_index);
            }
            println!("Format index at stamens: {}", format_index);
            println!("Anther string length: {}", anthers_string.chars().count());
            format_index += anthers_string.chars().count();

            anthers_string
        } else {
            "".into()
        };

        let carpels = if let Some(c) = &self.carpels {
            let carpels_string = format!(",{}", c);

            if adnation_vec.contains(&Part::Carpels) {
                adnation_status.carpels = Some(format_index);
            }
            println!("Format index at carpels: {}", format_index);
            println!("carpels string length: {}", carpels_string.chars().count());
            format_index += carpels_string.chars().count();

            carpels_string
        } else {
            "".into()
        };

        let fruits = &self
            .fruit
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let fruit_string = format!(";{}", fruits);
        let adnation_string = if adnation_status.to_string().is_empty() {
            "".to_string()
        } else {
            format!("\n{}", adnation_status)
        };

        println!("{:?}", adnation_status);
        write!(
            f,
            "{}{}{}{}{}{}",
            sym, calyx_perianth_or_tepals, anthers, carpels, fruit_string, adnation_string
        )
    }
}

/// An ovary can be inferior or
/// superior. Though, there are in
/// betweens.
#[derive(Debug)]
pub enum Ovary {
    Superior,
    Inferior,
}

/// The part of the flower, which
/// occurs as a whorl.
#[derive(Debug, Clone, PartialEq)]
pub enum Part {
    Tepals,
    Calyx,
    Petals,
    Stamens,
    Carpels,
}

impl FromStr for Part {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "T" => Ok(Self::Tepals),
            "K" => Ok(Self::Calyx),
            "C" => Ok(Self::Petals),
            "A" => Ok(Self::Stamens),
            "G" => Ok(Self::Carpels),
            err => Err(Error::new(ErrorKind::FromStr(format!(
                "The input string: {} - is not recognised",
                err
            )))),
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::Tepals => write!(f, "T"),
            Part::Calyx => write!(f, "K"),
            Part::Petals => write!(f, "C"),
            Part::Stamens => write!(f, "A"),
            Part::Carpels => write!(f, "G"),
        }
    }
}

/// Sterility status of an organ.
#[derive(Debug)]
pub enum Sterile {
    Fertile,
    Sterile,
}

impl Display for Sterile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sterile::Fertile => write!(f, ""),
            Sterile::Sterile => write!(f, "•"),
        }
    }
}

/// All the different fruit types.
#[derive(Debug)]
pub enum Fruit {
    Achene,
    Berry,
    Capsule, // note there are many different capsule types
    Caryopsis,
    DDrupe, // dehiscent drupe
    Drupe,
    Follicle,
    IPod, // indehiscent pod
    Legume,
    Loment,
    Nut,
    Pome,
    Samara,
    Schizocarp,
    Silique,
    Utricle,
    None,
}

// TODO: the fruit collection needs to be bigger, e.g. drupelets, berrylets...
impl Display for Fruit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fruit::Achene => write!(f, "achene"),
            Fruit::Berry => write!(f, "berry"),
            Fruit::Capsule => write!(f, "capsule"),
            Fruit::Caryopsis => write!(f, "caryopsis"),
            Fruit::DDrupe => write!(f, "dehiscent drupe"),
            Fruit::Drupe => write!(f, "drupe"),
            Fruit::Follicle => write!(f, "follicle"),
            Fruit::IPod => write!(f, "indehiscent pod"),
            Fruit::Legume => write!(f, "legume"),
            Fruit::Loment => write!(f, "loment"),
            Fruit::Nut => write!(f, "nut"),
            Fruit::Pome => write!(f, "pome"),
            Fruit::Samara => write!(f, "samara"),
            Fruit::Schizocarp => write!(f, "schizocarp"),
            Fruit::Silique => write!(f, "silique"),
            Fruit::Utricle => write!(f, "utricle"),
            Fruit::None => write!(f, "no fruit"),
        }
    }
}

impl FromStr for Fruit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "achene" => Ok(Self::Achene),
            "berry" => Ok(Self::Berry),
            "capsule" => Ok(Self::Capsule),
            "caryopsis" => Ok(Self::Caryopsis),
            "dehiscent drupe" => Ok(Self::DDrupe),
            "drupe" => Ok(Self::Drupe),
            "follicle" => Ok(Self::Follicle),
            "indehiscent pod" => Ok(Self::IPod),
            "legume" => Ok(Self::Legume),
            "loment" => Ok(Self::Loment),
            "nut" => Ok(Self::Nut),
            "pome" => Ok(Self::Pome),
            "samara" => Ok(Self::Samara),
            "schizocarp" => Ok(Self::Schizocarp),
            "silique" => Ok(Self::Silique),
            "utricle" => Ok(Self::Utricle),
            "-" | "" => Ok(Self::None),
            other => Err(Error::new(ErrorKind::FromStr(format!(
                "fruit: {}, not recognised",
                other
            )))),
        }
    }
}

/// An individual floral part
// need to think about how to incorporate whorls here
#[derive(Debug)]
pub struct FloralPart {
    /// Either the calyx, petals, stamens
    /// or carpels
    part: Part,
    /// Is the floral part connate?
    connate: bool,
    /// Is there variation in connation?
    connation_variation: bool,
    whorls: Vec<Whorl>,
}

impl FloralPart {
    pub fn set_part(&mut self, part: Part) {
        self.part = part;
    }
    pub fn set_connation(&mut self, connation: bool) {
        self.connate = connation;
    }
    pub fn set_connation_variation(&mut self, connation_variation: bool) {
        self.connation_variation = connation_variation;
    }
}

#[derive(Debug)]
pub struct Whorl {
    number: Option<FloralPartNumber>,
    /// The minimum number of floral parts
    min: Option<FloralPartNumber>,
    /// The maximum number of floral parts
    max: Option<FloralPartNumber>,
    /// The sterility status of a floral part.
    /// Weirdly I guess this applies to tepals/petals too
    sterile: Sterile,
}

impl Whorl {
    pub fn new(
        number: Option<FloralPartNumber>,
        min: Option<FloralPartNumber>,
        max: Option<FloralPartNumber>,
        sterile: Sterile,
    ) -> Self {
        Self {
            number,
            min,
            max,
            sterile,
        }
    }
}

impl Display for Whorl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let number_op = self.number.is_some();
        let min_op = self.min.is_some();
        let max_op = self.max.is_some();

        let number_or_range = match (number_op, min_op, max_op) {
            (true, false, false) => self.number.as_ref().unwrap().to_string(),
            (false, true, true) => format!(
                "{}-{}",
                self.min.as_ref().unwrap(),
                self.max.as_ref().unwrap()
            ),
            _ => panic!("either number, or min/max must be specified"),
        };

        let sterile = match self.sterile {
            Sterile::Fertile => "".into(),
            Sterile::Sterile => format!("{}", Sterile::Sterile),
        };

        write!(f, "{}{}", number_or_range, sterile)
    }
}

impl Default for FloralPart {
    fn default() -> Self {
        Self {
            part: Part::Tepals,
            connate: false,
            connation_variation: false,
            whorls: vec![],
        }
    }
}

impl Display for FloralPart {
    // connation and connation variation are not formatted yet.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // move to the Whorl struct.
        let mut whorl_strings = Vec::new();

        for whorl in &self.whorls {
            whorl_strings.push(whorl.to_string());
        }

        write!(f, "{}{}", self.part, whorl_strings.join("+"))
    }
}

impl FloralPart {
    // add a whorl into the struct
    // probably not necessary but whatever
    pub fn add_whorl(&mut self, whorl: Whorl) {
        self.whorls.push(whorl);
    }
}
