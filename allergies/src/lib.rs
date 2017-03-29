
#[derive(Clone,Debug,PartialEq)]
pub enum Allergen {
    Eggs = 0,
    Peanuts = 1,
    Shellfish = 2,
    Strawberries = 3,
    Tomatoes = 4,
    Chocolate = 5,
    Pollen = 6,
    Cats = 7,
}

impl Allergen {
    fn from_u8(x: u8) -> Self {
        match x {
            0 => Allergen::Eggs,
            1 => Allergen::Peanuts,
            2 => Allergen::Shellfish,
            3 => Allergen::Strawberries,
            4 => Allergen::Tomatoes,
            5 => Allergen::Chocolate,
            6 => Allergen::Pollen,
            _ => Allergen::Cats,
        }
    }

    #[inline]
    fn to_u8(&self) -> u8 {
        self.clone() as u8
    }
}

pub struct Allergies(u8);

impl Allergies {
    pub fn new(v: usize) -> Self {
        Allergies(v as u8)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        (0..8u8)
            .filter(|x| self.0 & (1 << x) as u8 > 0)
            .map(|i| Allergen::from_u8(i))
            .collect::<Vec<_>>()
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0 & (1 << allergen.to_u8()) > 0
    }
}