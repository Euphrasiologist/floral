use std::fmt::{self, Display};
use std::result;
use std::str::FromStr;

use crate::error::{Error, ErrorKind};

/// If the user wants an explanation of the floral parts
trait ExplainFloralFormula {
    /// The only method in this trait is to explain the
    /// input as a string
    fn explain(&self) -> String;
}

/// The type of flower we're looking at
#[derive(PartialEq, Hash, Eq, PartialOrd, Ord)]
pub enum FlowerType {
    /// Bisexual or perfect flowers
    Bisexual,
    /// Female only parts
    Carpellate,
    /// Male only parts
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

/// The floral symmetry of a flower
#[derive(Debug)]
pub enum Symmetry {
    /// Infinitely many symmetries
    Radial,
    /// One line of symmetry
    /// We encase a [`BilateralType`] as there are
    /// different kinds of bilateral symmetry
    Bilateral(BilateralType),
    /// No lines of symmetry
    Asymmetry,
    /// Spiral
    Spiral,
    /// Disymmetric
    Disymmetric,
}

/// The specific kind of bilateral symmetry
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
        if s == "-" || s.is_empty() {
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

/// Adnation describes floral fusion between different
/// floral parts
#[derive(Debug, Clone, Default)]
pub struct Adnation {
    /// Whether there is variation in adnation within the
    /// plant group described
    variation: bool,
    /// An optional vector of floral parts which are adnated
    parts: Option<Vec<Part>>,
}

impl Adnation {
    /// Set the variation of the adnation
    pub fn set_variation(&mut self, variation: bool) {
        self.variation = variation;
    }
    /// Push a floral part to the optional vector of floral
    /// parts
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
#[derive(Debug, Default)]
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
    /// Fruit
    fruit: Vec<Fruit>,
    /// Where is the adnation present?
    adnation: Adnation,
}

impl Formula {
    /// Constructor function for the symmetry
    pub fn with_symmetry(mut self, symmetry: Vec<Symmetry>) -> Formula {
        self.symmetry = symmetry;
        self
    }
    /// Constructor function for the tepals
    pub fn with_tepals(mut self, tepals: Option<FloralPart>) -> Formula {
        self.tepals = tepals;
        self
    }
    /// Constructor function for the sepals
    pub fn with_sepals(mut self, sepals: Option<FloralPart>) -> Formula {
        self.sepals = sepals;
        self
    }
    /// Constructor function for the petals
    pub fn with_petals(mut self, petals: Option<FloralPart>) -> Formula {
        self.petals = petals;
        self
    }
    /// Constructor function for the stamens
    pub fn with_stamens(mut self, stamens: Option<FloralPart>) -> Formula {
        self.stamens = stamens;
        self
    }
    /// Constructor function for the carpels
    pub fn with_carpels(mut self, carpels: Option<FloralPart>) -> Formula {
        self.carpels = carpels;
        self
    }
    /// Constructor function for the fruit
    pub fn with_fruit(mut self, fruit: Vec<Fruit>) -> Formula {
        self.fruit = fruit;
        self
    }
    /// Constructor function for the adnation
    pub fn with_adnation(mut self, adnation: Adnation) -> Formula {
        self.adnation = adnation;
        self
    }
    /// Build the floral formula. Might be redundant?
    pub fn build(self) -> Formula {
        Formula {
            symmetry: self.symmetry,
            tepals: self.tepals,
            sepals: self.sepals,
            petals: self.petals,
            stamens: self.stamens,
            carpels: self.carpels,
            fruit: self.fruit,
            adnation: self.adnation,
        }
    }
    pub fn has_adnation(&self) -> bool {
        self.adnation.parts.is_some()
    }
}

/// The information needed to render the adnation
/// in the display method of the floral formula
#[derive(Debug, Default)]
struct AdnationIndex {
    /// Whether adnation is variable or not.
    /// Inherited from [`Adnation`]
    variation: bool,
    /// Where the tepals index is
    tepals: Option<usize>,
    /// Where the sepals index is
    sepals: Option<usize>,
    /// Where the petals index is
    petals: Option<usize>,
    /// Where the stamens index is
    stamens: Option<usize>,
    /// Where the carpels index is
    carpels: Option<usize>,
}

trait AdnationVariation {
    const CONSTANT: [char; 4];
    const VARIABLE: [char; 4];
}

impl AdnationVariation for AdnationIndex {
    /// The character set for a constant (i.e. invariant)
    /// adnation between floral parts
    const CONSTANT: [char; 4] = ['╰', '╯', '─', '┴'];

    /// The character set for variable adnation between
    /// floral parts
    const VARIABLE: [char; 4] = ['└', '┘', '┄', '┴'];
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
        // get the character set for the andation variation drawing
        let character_set = if self.variation {
            Self::VARIABLE
        } else {
            Self::CONSTANT
        };
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
                adnation.push(character_set[0]);
                for _ in merged_only_some[0]..merged_only_some[1] - 1 {
                    adnation.push(character_set[2]);
                }
                adnation.push(character_set[1]);
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
                        adnation.push(character_set[0]);
                        // and continue to next loop iteration
                        continue;
                    }

                    // dashes in between
                    for _ in merged_only_some[fusion - 1]..merged_only_some[fusion] - 1 {
                        adnation.push(character_set[2]);
                    }

                    // and we close on the last iteration
                    if adnation_iter.peek().is_none() {
                        adnation.push(character_set[1]);
                        // just to be sure.
                        break;
                    }
                    // and if we are in between iterations,
                    // we want this funky char
                    adnation.push(character_set[3]);
                }
                write!(f, "{}", adnation)
            }
            _ => Err(fmt::Error),
        }
    }
}

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

        let adnation_vec = self.adnation.parts.clone().unwrap_or(vec![]);
        let mut adnation_status = AdnationIndex::default();
        // set the adnation status. This is a really horrible way
        // of handing that info on, but oh well.
        if self.adnation.variation {
            // we know that there is adnation
            adnation_status.variation = true;
        }

        let calyx_perianth_or_tepals = if let Some(t) = &self.tepals {
            // calyx and petals not needed
            // probably make this into a proper error.
            assert!(self.sepals.is_none() && self.petals.is_none());
            // check if tepals are in the adnation vec.
            if adnation_vec.contains(&Part::Tepals) {
                // if there is connation, there's an extra parenthesis
                // we have to account for.
                // same for other floral parts.
                if t.connate {
                    // so we add one to the index
                    format_index += 1;
                    // assign to the adnation status
                    adnation_status.tepals = Some(format_index);
                    // then decrement the index again
                    format_index -= 1;
                } else {
                    adnation_status.tepals = Some(format_index);
                }
            }
            // the tepal string to return
            let tepal_string = format!(",{}", t);
            // increment the format index
            format_index += tepal_string.chars().count();
            tepal_string
        } else {
            // these unwraps are safe.
            let calyx = self.sepals.as_ref().unwrap();
            let petals = self.petals.as_ref().unwrap();
            // make calyx string here
            let calyx_string = format!(",{}", calyx);
            let petals_string = format!(",{}", petals);

            // deal with adnation logic here
            if adnation_vec.contains(&Part::Calyx) {
                if calyx.connate {
                    format_index += 1;
                    adnation_status.sepals = Some(format_index);
                    format_index -= 1;
                } else {
                    adnation_status.sepals = Some(format_index);
                }
            }
            // increment the format index
            format_index += calyx_string.chars().count();
            if adnation_vec.contains(&Part::Petals) {
                if petals.connate {
                    format_index += 1;
                    adnation_status.petals = Some(format_index);
                    format_index -= 1;
                } else {
                    adnation_status.petals = Some(format_index);
                }
            }
            // increment the format index again
            format_index += petals_string.chars().count();
            format!("{}{}", calyx_string, petals_string)
        };

        let anthers = if let Some(a) = &self.stamens {
            let anthers_string = format!(",{}", a);

            if adnation_vec.contains(&Part::Stamens) {
                if a.connate {
                    format_index += 1;
                    adnation_status.stamens = Some(format_index);
                    format_index -= 1;
                } else {
                    adnation_status.stamens = Some(format_index);
                }
            }
            format_index += anthers_string.chars().count();

            anthers_string
        } else {
            "".into()
        };

        let carpels = if let Some(c) = &self.carpels {
            let carpels_string = format!(",{}", c);

            if adnation_vec.contains(&Part::Carpels) {
                if c.connate {
                    format_index += 1;
                    adnation_status.carpels = Some(format_index);
                } else {
                    adnation_status.carpels = Some(format_index);
                }
            }
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
#[derive(Debug, Clone, Copy)]
pub enum Ovary {
    /// A superior ovary
    Superior,
    /// An inferior ovary
    Inferior,
    /// Both
    Both,
}

impl FromStr for Ovary {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "s" => Ok(Self::Superior),
            "i" => Ok(Self::Inferior),
            ov_str => Err(Error::new(ErrorKind::FromStr(format!(
                "the string: {}, does not correspond to an ovary position",
                ov_str
            )))),
        }
    }
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

/// All the different fruit types. A growing list.
#[derive(Debug)]
pub enum Fruit {
    Achene,
    Berry,
    Berrylets,
    Capsule, // note there are many different capsule types
    Caryopsis,
    DDrupe, // dehiscent drupe
    Drupe,
    Drupelets,
    Follicle,
    IPod, // indehiscent pod
    Legume,
    Loment,
    Nut,
    AggregateOfNuts,
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
            Fruit::Berrylets => write!(f, "berrylets"),
            Fruit::Capsule => write!(f, "capsule"),
            Fruit::Caryopsis => write!(f, "caryopsis"),
            Fruit::DDrupe => write!(f, "dehiscent drupe"),
            Fruit::Drupe => write!(f, "drupe"),
            Fruit::Drupelets => write!(f, "drupelets"),
            Fruit::Follicle => write!(f, "follicle"),
            Fruit::IPod => write!(f, "indehiscent pod"),
            Fruit::Legume => write!(f, "legume"),
            Fruit::Loment => write!(f, "loment"),
            Fruit::Nut => write!(f, "nut"),
            Fruit::AggregateOfNuts => write!(f, "aggregate of nuts"),
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
            "berry" | "berries" => Ok(Self::Berry),
            "berrylets" => Ok(Self::Berrylets),
            "capsule" | "fleshy capsule" => Ok(Self::Capsule),
            "caryopsis" => Ok(Self::Caryopsis),
            "dehiscent drupe" => Ok(Self::DDrupe),
            "drupe" | "drupes" => Ok(Self::Drupe),
            "drupelets" => Ok(Self::Drupelets),
            "follicle" | "follicles" => Ok(Self::Follicle),
            "indehiscent pod" => Ok(Self::IPod),
            "legume" => Ok(Self::Legume),
            "loment" => Ok(Self::Loment),
            "nut" => Ok(Self::Nut),
            "aggregate of nuts" => Ok(Self::AggregateOfNuts),
            "pome" => Ok(Self::Pome),
            "samara" | "samaras" => Ok(Self::Samara),
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
#[derive(Debug)]
pub struct FloralPart {
    /// Either the calyx, petals, stamens
    /// or carpels
    part: Part,
    /// Is the floral part connate?
    connate: bool,
    /// Is there variation in connation?
    connation_variation: bool,
    /// All the whorls in this floral part which are
    /// differentiated
    whorls: Vec<Whorl>,
    /// Ovary information makes most sense here
    ovary: Option<Ovary>,
}

impl FloralPart {
    /// Set the floral part
    pub fn set_part(&mut self, part: Part) {
        self.part = part;
    }
    /// Set the connation of the floral part
    pub fn set_connation(&mut self, connation: bool) {
        self.connate = connation;
    }
    /// Set the connation variation of the floral part
    pub fn set_connation_variation(&mut self, connation_variation: bool) {
        self.connation_variation = connation_variation;
    }
    pub fn set_ovary(&mut self, ovary: Option<Ovary>) {
        self.ovary = ovary;
    }
}

/// A part of a floral organ, within the same part
#[derive(Debug)]
pub struct Whorl {
    /// The number of floral parts (if there is no range)
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
    /// Constructor for the [`Whorl`] struct
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
            ovary: None,
        }
    }
}

// TODO: deal with fusion between different whorls of same floral part?
impl Display for FloralPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut whorl_strings = Vec::new();

        for whorl in &self.whorls {
            whorl_strings.push(whorl.to_string());
        }

        // TODO: HERE IS WHERE THE OVARY POSITION GOES
        let part = if let Some(ovary) = self.ovary {
            match ovary {
                Ovary::Superior => format!("{}{}", '\u{0332}', self.part),
                Ovary::Inferior => format!("{}{}", '\u{305}', self.part),
                Ovary::Both => format!("{}{}{}", '\u{305}', '\u{0332}', self.part),
            }
        } else {
            self.part.to_string()
        };

        // connation is () around the floral part.
        // variation is denoted as (].
        match (self.connate, self.connation_variation) {
            (true, true) => write!(f, "({}{}]", part, whorl_strings.join("+")),
            (true, false) => write!(f, "({}{})", part, whorl_strings.join("+")),
            (false, _) => write!(f, "{}{}", part, whorl_strings.join("+")),
        }
    }
}

impl FloralPart {
    /// Add a whorl into the floral part.
    pub fn add_whorl(&mut self, whorl: Whorl) {
        self.whorls.push(whorl);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn floral_from_test_str(s: &str) -> Formula {
        let line_element = s.split(',').collect::<Vec<&str>>();

        if let [order, family, flower_type, symmetry, tepals, calyx, petals, anthers, carpels, ovary, fruit, adnation] =
            &line_element[..]
        {
            crate::parse::floral_from_str(
                symmetry, tepals, calyx, petals, anthers, carpels, ovary, fruit, adnation,
            )
            .unwrap()
        } else {
            panic!("could not parse floral string")
        }
    }

    #[test]
    fn test_1() {
        // a simple case
        // order, family, flower type, symmetry, tepals, calyx, petals, anthers, carpels, ovary, fruit, adnation
        let floral_string = "Amborellales,amborellaceae,s,s,8-11,-,-,inf,0,-,-,-";
        let fs = floral_from_test_str(floral_string);
        assert_eq!(fs.to_string(), "↻,T8-11,A∞,G0;no fruit")
    }
    #[test]
    fn test_2() {
        // bisexual with berry fruit
        // order, family, flower type, symmetry, tepals, calyx, petals, anthers, carpels, ovary, fruit, adnation
        let floral_string = "test2,test2,b,r,2,-,-,2,2,i,berry,-";
        let fs = floral_from_test_str(floral_string);
        assert_eq!(fs.to_string(), "*,T2,A2,\u{305}G2;berry")
    }
    #[test]
    fn test_3() {
        // with some adnation between floral parts
        // order, family, flower type, symmetry, tepals, calyx, petals, anthers, carpels, ovary, fruit, adnation
        let floral_string = "test3,test3,b,r,2,-,-,2,2,i,berry,T;A;G";
        let fs = floral_from_test_str(floral_string);
        assert_eq!(
            fs.to_string(),
            "\
*,T2,A2,\u{305}G2;berry
  ╰──┴──╯"
        )
    }
    #[test]
    fn test_4() {
        // with some adnation between floral parts and fusion within
        // order, family, flower type, symmetry, tepals, calyx, petals, anthers, carpels, ovary, fruit, adnation
        let floral_string = "test4,test4,b,r,2;f,-,-,2;f,2;f,i,berry,T;A;G";
        let fs = floral_from_test_str(floral_string);
        assert_eq!(
            fs.to_string(),
            "\
*,(T2),(A2),(\u{305}G2);berry
   ╰────┴────╯"
        )
    }
    #[test]
    fn test_5() {
        // with some adnation between floral parts and fusion within
        // and extra whorls, in this case, 5 staminodes
        // order, family, flower type, symmetry, tepals, calyx, petals, anthers, carpels, ovary, fruit, adnation
        let floral_string = "test5,test5,b,r,2;f,-,-,2;5s;f,2;f,i,berry,T;A;G";
        let fs = floral_from_test_str(floral_string);
        assert_eq!(
            fs.to_string(),
            "\
*,(T2),(A2+5•),(\u{305}G2);berry
   ╰────┴───────╯"
        )
    }
}
