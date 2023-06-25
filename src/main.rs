/// An implementation of floral formulas
/// following Judd et al., 2002.
use std::fmt::Display;

/// The floral symmetry
enum Symmetry {
    /// Infinitely many symmetries
    Radial,
    /// One line of symmetry
    Bilateral,
    /// No lines of symmetry
    Asymmetry,
}

impl Display for Symmetry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symmetry::Radial => write!(f, "*"),
            Symmetry::Bilateral => write!(f, "X"),
            Symmetry::Asymmetry => write!(f, "↯"),
        }
    }
}

enum FloralPartNumber {
    Finite(u32),
    Infinite,
}

impl Display for FloralPartNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FloralPartNumber::Finite(u) => write!(f, "{}", u),
            FloralPartNumber::Infinite => write!(f, "∞"),
        }
    }
}

/// The total floral formula
struct Formula {
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
    fruit: Option<Vec<Fruit>>,
}

impl Display for Formula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sym = &self
            .symmetry
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(" or ");

        let calyx_perianth_or_tepals = if let Some(t) = &self.tepals {
            // calyx and petals not needed
            // probably make this into a proper error.
            assert!(self.sepals.is_none() && self.petals.is_none());
            format!(",{}", t)
        } else {
            let calyx = self.sepals.as_ref().unwrap();
            let petals = self.petals.as_ref().unwrap();
            format!(",{},{}", calyx, petals)
        };

        let anthers = if let Some(a) = &self.stamens {
            format!(",{}", a)
        } else {
            "".into()
        };

        let carpels = if let Some(c) = &self.carpels {
            format!(",{}", c)
        } else {
            "".into()
        };

        let fruit = if let Some(f) = &self.fruit {
            let fruits = f
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(",");
            format!(";{}", fruits)
        } else {
            "".into()
        };

        write!(
            f,
            "{}{}{}{}{}",
            sym, calyx_perianth_or_tepals, anthers, carpels, fruit
        )
    }
}

/// An ovary can be inferior or
/// superior. Though, there are in
/// betweens.
enum Ovary {
    Superior,
    Inferior,
}

/// The part of the flower, which
/// occurs as a whorl.
enum Part {
    Tepals,
    Calyx,
    Petals,
    Stamens,
    Carpels,
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
enum Sterile {
    Fertile,
    Sterile,
}

impl Display for Sterile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sterile::Fertile => write!(f, ""),
            Sterile::Sterile => write!(f, "●"),
        }
    }
}

/// All the different fruit types.
enum Fruit {
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
}

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
        }
    }
}

/// An individual floral part
struct FloralPart {
    /// Either the calyx, petals, stamens
    /// or carpels
    part: Part,
    /// Is the floral part connate?
    connate: bool,
    /// What's the number of this floral
    /// part? Optional because this part
    /// might not occur.
    number: Option<FloralPartNumber>,
    /// The minimum number of floral parts
    min: Option<FloralPartNumber>,
    /// The maximum number of floral parts
    max: Option<FloralPartNumber>,
    /// If the whorl is differentiated, how
    /// many are? And are they sterile (e.g. staminodes)?
    differentiation: Option<(FloralPartNumber, Sterile)>,
}

impl Display for FloralPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

        let diff = if let Some((n, sterile)) = &self.differentiation {
            match sterile {
                Sterile::Fertile => format!("+{}", n),
                Sterile::Sterile => format!("+{}{}", n, sterile),
            }
        } else {
            "".into()
        };

        write!(f, "{}{}{}", self.part, number_or_range, diff)
    }
}

fn main() {
    // let's try Annonaceae:
    let annonaceae = Formula {
        symmetry: vec![Symmetry::Radial],
        tepals: None,
        sepals: Some(FloralPart {
            part: Part::Calyx,
            connate: false,
            number: Some(FloralPartNumber::Finite(3)),
            min: None,
            max: None,
            differentiation: None,
        }),
        petals: Some(FloralPart {
            part: Part::Petals,
            connate: false,
            number: Some(FloralPartNumber::Finite(6)),
            min: None,
            max: None,
            differentiation: None,
        }),
        stamens: Some(FloralPart {
            part: Part::Stamens,
            connate: false,
            number: Some(FloralPartNumber::Infinite),
            min: None,
            max: None,
            differentiation: None,
        }),
        carpels: Some(FloralPart {
            part: Part::Carpels,
            connate: false,
            number: None,
            min: Some(FloralPartNumber::Finite(3)),
            max: Some(FloralPartNumber::Infinite),
            differentiation: None,
        }),
        ovary: Some(Ovary::Superior),
        fruit: Some(vec![Fruit::Berry]),
    };
    println!("Annonnaceae floral Formula: {}", annonaceae);
}
