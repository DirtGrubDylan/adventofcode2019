use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, PartialEq)]
struct Ingredient {
    name: String,
    quantity: u64,
}

impl Ingredient {
    pub fn new(name: &str, quantity: u64) -> Ingredient {
        Ingredient {
            name: String::from(name),
            quantity,
        }
    }

    pub fn new_from_str(ingredient_str: &str) -> Ingredient {
        let split_str: Vec<&str> = ingredient_str.split(' ').collect();

        let quantity = split_str.get(0).unwrap().parse().unwrap();
        let name = split_str.get(1).unwrap().to_string();

        Ingredient { name, quantity }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Reaction {
    reaction_type: String,
    produces: u64,
    ingredients: Vec<Ingredient>,
}

impl Reaction {
    pub fn new_from_str(reaction_str: &str) -> Reaction {
        let temp: Vec<&str> = reaction_str.split(" => ").collect();

        let reaction_ingredient = Ingredient::new_from_str(temp.get(1).unwrap());

        let (reaction_type, produces) = (reaction_ingredient.name, reaction_ingredient.quantity);

        let ingredients: Vec<Ingredient> = temp
            .get(0)
            .unwrap()
            .split(", ")
            .into_iter()
            .map(|s| Ingredient::new_from_str(s))
            .collect();

        Reaction {
            reaction_type,
            produces,
            ingredients,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Nanofactory {
    reactions: HashMap<String, Reaction>,
    leftovers: HashMap<String, u64>,
    ore_required: u64,
    processing_queue: VecDeque<Ingredient>,
}

impl Nanofactory {
    pub fn new() -> Nanofactory {
        let reactions = HashMap::new();
        let leftovers = HashMap::new();
        let processing_queue = VecDeque::new();
        let ore_required = 0;

        Nanofactory {
            reactions,
            leftovers,
            ore_required,
            processing_queue,
        }
    }

    pub fn add_reaction_from_str(&mut self, reaction_str: &str) {
        let reaction = Reaction::new_from_str(reaction_str);

        self.reactions
            .insert(reaction.reaction_type.clone(), reaction);
    }

    pub fn get_leftovers(&self) -> HashMap<String, u64> {
        self.leftovers.clone()
    }

    pub fn number_of_ore_to_make_n_fuel(&self, fuel_quantity: u64) -> u64 {
        let mut simulated_self = self.clone();

        simulated_self.make_n_fuel(fuel_quantity);

        simulated_self.ore_required
    }

    fn make_n_fuel(&mut self, fuel_quantity: u64) {
        // add fuel ingredient w/ quantity to queue
        let fuel_ingredient = Ingredient::new("FUEL", fuel_quantity);

        self.processing_queue.push_back(fuel_ingredient);

        while let Some(next_ingredient) = self.processing_queue.pop_front() {
            if next_ingredient.name == "ORE" {
                self.ore_required += next_ingredient.quantity;

                continue;
            }

            let mut needed_ingredients = self.get_ingredients_needed_to_make(next_ingredient);

            self.processing_queue.append(&mut needed_ingredients);
        }
    }

    fn get_ingredients_needed_to_make(&mut self, ingredient: Ingredient) -> VecDeque<Ingredient> {
        let leftovers_of_ingredient = self.leftovers.entry(ingredient.name.clone()).or_insert(0);

        // Subtract all possible leftovers from ingredient quantity
        let needed_ingredient_quantity =
            (ingredient.quantity).saturating_sub(*leftovers_of_ingredient);

        // remove leftovers used
        *leftovers_of_ingredient = (*leftovers_of_ingredient).saturating_sub(ingredient.quantity);

        // if we dont need anything, return empty queue
        if needed_ingredient_quantity == 0 {
            return VecDeque::new();
        }

        // get reaction
        let reaction_to_make_ingredient = self.reactions.get(&ingredient.name).unwrap();

        // determine the quanity of reactions needed
        let quantity_of_ingredient_that_can_be_made = reaction_to_make_ingredient.produces;

        let mut number_of_reactions_needed =
            needed_ingredient_quantity / quantity_of_ingredient_that_can_be_made;

        if needed_ingredient_quantity % quantity_of_ingredient_that_can_be_made != 0 {
            number_of_reactions_needed += 1;
        }

        // determine any leftovers
        *leftovers_of_ingredient = number_of_reactions_needed
            * quantity_of_ingredient_that_can_be_made
            - needed_ingredient_quantity;

        // multiply the number of reaction to each ingredient of reaction
        reaction_to_make_ingredient
            .ingredients
            .iter()
            .map(|needed_ingredient| {
                Ingredient::new(
                    &needed_ingredient.name.clone(),
                    needed_ingredient.quantity * number_of_reactions_needed,
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const REACTION_INFO: [&str; 7] = [
        "9 ORE => 2 A",
        "8 ORE => 3 B",
        "7 ORE => 5 C",
        "3 A, 4 B => 1 AB",
        "5 B, 7 C => 1 BC",
        "4 C, 1 A => 1 CA",
        "2 AB, 3 BC, 4 CA => 1 FUEL",
    ];

    const ORE_NEEDED: u64 = 165;

    #[test]
    fn test_new_ingredient_from_str() {
        let expected = Ingredient::new("BC", 3);

        let result = Ingredient::new_from_str("3 BC");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_fuel_reaction_from_str() {
        let expected = Reaction {
            reaction_type: String::from("FUEL"),
            produces: 1,
            ingredients: vec![
                Ingredient::new("AB", 2),
                Ingredient::new("BC", 3),
                Ingredient::new("CA", 4),
            ],
        };

        let result = Reaction::new_from_str(REACTION_INFO[6]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_c_reaction_from_str() {
        let expected = Reaction {
            reaction_type: String::from("C"),
            produces: 5,
            ingredients: vec![Ingredient::new("ORE", 7)],
        };

        let result = Reaction::new_from_str(REACTION_INFO[2]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_reaction_from_str() {
        let mut nanofactory = Nanofactory::new();

        let expected_hashmap: HashMap<String, Reaction> = vec![
            (
                String::from("FUEL"),
                Reaction::new_from_str("2 AB, 3 BC, 4 CA => 1 FUEL"),
            ),
            (String::from("C"), Reaction::new_from_str("7 ORE => 5 C")),
        ]
        .into_iter()
        .collect();

        nanofactory.add_reaction_from_str("2 AB, 3 BC, 4 CA => 1 FUEL");
        nanofactory.add_reaction_from_str("7 ORE => 5 C");

        assert_eq!(nanofactory.reactions, expected_hashmap);
    }

    #[test]
    fn test_get_ingredients_needed_to_make_1_fuel() {
        let mut nanofactory = Nanofactory::new();

        for reaction_str in &REACTION_INFO {
            nanofactory.add_reaction_from_str(reaction_str);
        }

        let fuel_ingredient = Ingredient::new("FUEL", 1);

        let expected: VecDeque<Ingredient> = vec![
            Ingredient::new("AB", 2),
            Ingredient::new("BC", 3),
            Ingredient::new("CA", 4),
        ]
        .into_iter()
        .collect();

        let result = nanofactory.get_ingredients_needed_to_make(fuel_ingredient);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_ingredients_needed_to_make_3_bc() {
        let mut nanofactory = Nanofactory::new();

        for reaction_str in &REACTION_INFO {
            nanofactory.add_reaction_from_str(reaction_str);
        }

        let bc_ingredient = Ingredient::new("BC", 3);

        let expected: VecDeque<Ingredient> =
            vec![Ingredient::new("B", 15), Ingredient::new("C", 21)]
                .into_iter()
                .collect();

        let result = nanofactory.get_ingredients_needed_to_make(bc_ingredient);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_ingredients_needed_to_make_21_c() {
        let mut nanofactory = Nanofactory::new();

        for reaction_str in &REACTION_INFO {
            nanofactory.add_reaction_from_str(reaction_str);
        }

        let c_ingredient = Ingredient::new("C", 21);

        let expected: VecDeque<Ingredient> = vec![Ingredient::new("ORE", 35)].into_iter().collect();

        let result = nanofactory.get_ingredients_needed_to_make(c_ingredient);

        assert_eq!(result, expected);
        assert!(nanofactory.leftovers.contains_key(&String::from("C")));
        assert_eq!(nanofactory.leftovers.get(&String::from("C")), Some(&4));
    }

    #[test]
    fn test_get_ingredients_needed_to_make_16_c_with_4_leftovers() {
        let mut nanofactory = Nanofactory::new();

        for reaction_str in &REACTION_INFO {
            nanofactory.add_reaction_from_str(reaction_str);
        }

        nanofactory.leftovers = vec![(String::from("C"), 4)].into_iter().collect();

        let c_ingredient = Ingredient::new("C", 16);

        let expected: VecDeque<Ingredient> = vec![Ingredient::new("ORE", 21)].into_iter().collect();

        let result = nanofactory.get_ingredients_needed_to_make(c_ingredient);

        assert_eq!(result, expected);
        assert!(nanofactory.leftovers.contains_key(&String::from("C")));
        assert_eq!(nanofactory.leftovers.get(&String::from("C")), Some(&3));
    }

    #[test]
    fn test_number_of_ore_to_make_n_fuel() {
        let mut nanofactory = Nanofactory::new();

        for reaction_str in &REACTION_INFO {
            nanofactory.add_reaction_from_str(reaction_str);
        }

        let result = nanofactory.number_of_ore_to_make_n_fuel(1);

        assert_eq!(result, ORE_NEEDED);
    }
}
