#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

pub struct Allergies {
    allergens: Vec<Allergen>,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut allergens = Vec::new();

        if score == 0 {
            return Allergies { allergens };
        }

        let score = if score > 255 { score % 256 } else { score };

        if score & (1 << 0) != 0 {
            allergens.push(Allergen::Eggs);
        }
        if score & (1 << 1) != 0 {
            allergens.push(Allergen::Peanuts);
        }
        if score & (1 << 2) != 0 {
            allergens.push(Allergen::Shellfish);
        }
        if score & (1 << 3) != 0 {
            allergens.push(Allergen::Strawberries);
        }
        if score & (1 << 4) != 0 {
            allergens.push(Allergen::Tomatoes);
        }
        if score & (1 << 5) != 0 {
            allergens.push(Allergen::Chocolate);
        }
        if score & (1 << 6) != 0 {
            allergens.push(Allergen::Pollen);
        }
        if score & (1 << 7) != 0 {
            allergens.push(Allergen::Cats);
        }

        Allergies { allergens }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.clone()
    }
}
