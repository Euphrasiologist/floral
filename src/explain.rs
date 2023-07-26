use crate::floral::*;
use textwrap;

/// If the user wants an explanation of the floral parts
pub trait ExplainFloralFormula {
    /// The only method in this trait is to explain the
    /// input as a string
    fn explain(&self) -> String;
}

impl ExplainFloralFormula for BilateralType {
    fn explain(&self) -> String {
        match self {
            BilateralType::Up => format!("upwards bilateral (X({}))", self),
            BilateralType::Down => format!("downward bilateral (X({}))", self),
            BilateralType::Left => format!("left bilateral (X({}))", self),
            BilateralType::Right => format!("right bilateral (X({}))", self),
            BilateralType::Upleft => format!("up and left bilateral (X({}))", self),
            BilateralType::Upright => format!("up and right bilateral (X({}))", self),
            BilateralType::Downleft => format!("down and left bilateral (X({}))", self),
            BilateralType::Downright => format!("down and right bilateral (X({}))", self),
        }
    }
}

impl ExplainFloralFormula for FloralPartNumber {
    fn explain(&self) -> String {
        match self {
            FloralPartNumber::Finite(num) => format!("{}", num),
            FloralPartNumber::Fractional(_) => "½".into(),
            FloralPartNumber::Infinite => "infinite".into(),
        }
    }
}

impl ExplainFloralFormula for FlowerType {
    fn explain(&self) -> String {
        match self {
            FlowerType::Bisexual => {
                "A bisexual flower with both male (androecium) and female (gynoecium) parts.".into()
            }
            FlowerType::Carpellate => "A carpellate (female only) flower.".into(),
            FlowerType::Staminate => "A staminate (male only) flower.".into(),
        }
    }
}

impl ExplainFloralFormula for Fruit {
    fn explain(&self) -> String {
        match self {
            Fruit::Achene => format!("{} - small, dry, indehiscent, single seeded, thin walled", self),
            Fruit::Berry => format!("{} - fleshy, indehiscent, one to many seeded, sometimes heterogeneous (i.e. inner fleshy, outer leathery)", self),
            Fruit::Berrylets => format!("{} - as a berry, but an aggregate (i.e. developed from multiple carpels)", self),
            Fruit::Capsule => format!("{} - dry (rarely fleshy), dehiscent, two to many seeded", self),
            Fruit::Caryopsis => format!("{} - small, dry, indehiscent, with wall surrounding and fused to seed (grass specific)", self),
            Fruit::DDrupe => format!("{} - fleshy, indehiscent, outer part soft to fibrous, breaking apart to reveal nut-like pits", self),
            Fruit::Drupe => format!("{} - fleshy, indehiscent, with one or more hard pits", self),    
            Fruit::Drupelets => format!("{} - as a drupe, but an aggregate (i.e. developed from multiple carpels)", self),
            Fruit::Follicle => format!("{} - dry to fleshy, from single carpel, releasing along a single longitudinal slit", self),
            Fruit::IPod => format!("{} - dry, indehiscent, few to many seeds", self),
            Fruit::Legume => format!("{} - dry, from single carpel that opens along two longitudinal slits (mainly legumes)", self),
            Fruit::Loment => format!("{} - dry, from single carpel that transversely breaks into single seeded units", self),
            Fruit::Nut => format!("{} - dry, indehiscent, large, with thick and bony wall around a single seed", self),
            Fruit::AggregateOfNuts => format!("{} - as a nut, but an aggregate (i.e. developed from multiple carpels)", self),
            Fruit::Pome => format!("{} - fleshy, indehiscent, with soft outer part, and papery structure around seeds", self),
            Fruit::Samara => format!("{} - dry, indehiscent, winged, one to two seeds", self),
            Fruit::Schizocarp => format!("{} - dry to fleshy, from two to many carpels that dehisces into mericarps (one to two seeded)", self),
            Fruit::Silique => format!("{} - dehiscent, derived from two carpels, with two halves splitting from a partition", self),
            Fruit::Utricle => format!("{} - dry, indehiscent, small, with thin wall that is loose and freen from a single seed", self),
            Fruit::None => format!("{} - no fruit to describe", self),
        }
    }
}

impl ExplainFloralFormula for Ovary {
    fn explain(&self) -> String {
        match self {
            Ovary::Superior => "a superior ovary".into(),
            Ovary::Inferior => "an inferior ovary".into(),
            Ovary::Both => "both superior and inferior ovaries".into(),
        }
    }
}

impl ExplainFloralFormula for Part {
    fn explain(&self) -> String {
        match self {
            Part::Tepals => "tepals".into(),
            Part::Calyx => "calyx".into(),
            Part::Petals => "petals".into(),
            Part::Stamens => "stamens".into(),
            Part::Carpels => "carpels".into(),
        }
    }
}

impl ExplainFloralFormula for Sterile {
    fn explain(&self) -> String {
        match self {
            Sterile::Fertile => "fertile part".into(),
            Sterile::Sterile => "sterile part".into(),
        }
    }
}

impl ExplainFloralFormula for Symmetry {
    fn explain(&self) -> String {
        match self {
            Symmetry::Radial => format!("radial ({})", self),
            Symmetry::Bilateral(b) => b.explain(),
            Symmetry::Asymmetry => format!("asymmetrical ({})", self),
            Symmetry::Spiral => format!("sprial ({})", self),
            Symmetry::Disymmetric => format!("disymmetric ({})", self),
        }
    }
}

impl ExplainFloralFormula for Adnation {
    fn explain(&self) -> String {
        let var = self.to_owned().get_variation();
        let parts = self.to_owned().get_parts();

        if let Some(inner_part_vec) = parts {
            let inner_parts = inner_part_vec
                .iter()
                .map(|e| e.explain())
                .collect::<Vec<String>>()
                .join(", and ");

            format!(
                "Adnation between {} floral parts. {} between species.",
                inner_parts,
                if var.to_owned() {
                    "Variable"
                } else {
                    "Not variable"
                }
            )
        } else {
            "There is no adnation between floral parts".into()
        }
    }
}

impl ExplainFloralFormula for FloralPart {
    fn explain(&self) -> String {
        // few more bits to this one.
        let part = self.get_part();
        let connation = match self.get_connation() {
            true => "connate",
            false => "not connate",
        };
        let connation_variation = match self.get_connation_variation() {
            true => "with variation in the connation",
            false => "with no variation in the connation",
        };
        let whorls = self.get_whorls();
        let mut whorls_string = String::new();
        for (mut idx, whorl) in whorls.iter().enumerate() {
            idx += 1;
            whorls_string += "\tWhorl ";
            whorls_string += &idx.to_string();
            whorls_string += ": ";
            whorls_string += &whorl.explain();
            whorls_string += "\n";
        }

        let ovary = match part {
            Part::Carpels => {
                let ovary = self.get_ovary();
                match ovary {
                    Some(o) => format!("\tWhorl has {}", o.explain()),

                    None => "".into(),
                }
            }
            _ => "".into(),
        };

        format!(
            "{} = {} are {} {}\n{}{}\n",
            self,
            part.explain(),
            connation,
            connation_variation,
            whorls_string,
            ovary
        )
    }
}

impl ExplainFloralFormula for Whorl {
    fn explain(&self) -> String {
        let min = self.get_min();
        let max = self.get_max();
        let number = self.get_number();
        let sterile = self.get_sterility().explain();

        match (min, max, number) {
            (None, None, Some(num)) => {
                format!("{} and has {} parts", sterile, num.explain())
            }
            (Some(min_n), Some(max_n), None) => {
                format!(
                    "{} and has between {} and {} parts",
                    sterile,
                    min_n.explain(),
                    max_n.explain()
                )
            }
            _ => "INVALID WHORL - this is a BUG!".to_string(),
        }
    }
}

// the actual explanation!
impl ExplainFloralFormula for Formula {
    fn explain(&self) -> String {
        let symmetry = self.get_symmetry();
        let tepals = self.get_tepals();
        let sepals = self.get_sepals();
        let petals = self.get_petals();
        let stamens = self.get_stamens();
        let carpels = self.get_carpels();
        let fruits = self.get_fruit();
        let adnation = self.get_adnation();

        let sym = symmetry
            .iter()
            .map(|e| e.explain())
            .collect::<Vec<String>>()
            .join(" or ");

        let symmetry_string = format!("The symmetry is {}", sym);

        // reduce a bit of boiler plate here...
        fn explain_floral_part(fp: &Option<FloralPart>) -> String {
            if let Some(p) = fp {
                p.explain()
            } else {
                "".into()
            }
        }

        let tepal_string = explain_floral_part(tepals);
        let sepal_string = explain_floral_part(sepals);
        let petal_string = explain_floral_part(petals);
        let stamen_string = explain_floral_part(stamens);
        let carpel_string = explain_floral_part(carpels);

        let fruit_string_inner = fruits
            .iter()
            .map(|e| e.explain())
            .collect::<Vec<String>>()
            .join("\n\t");

        let fruit_string = format!("Fruit(s):\n\t{}", fruit_string_inner);
        let adnation_string = adnation.explain();

        let out = format!(
            "\
{formula}

Explanation of floral formula above:

{symmetry}

{tepals}{sepals}{petals}{stamens}{carpels}
{fruits}

{adnation}",
            formula = self,
            symmetry = symmetry_string,
            tepals = tepal_string,
            sepals = sepal_string,
            petals = petal_string,
            stamens = stamen_string,
            carpels = carpel_string,
            fruits = fruit_string,
            adnation = adnation_string
        );
        let wrapped = textwrap::wrap(&out, 70);

        wrapped.join("\n")
    }
}
