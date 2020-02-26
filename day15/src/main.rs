use std::cmp;

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn new(capacity: i32, durability: i32, flavor: i32, texture: i32, calories: i32) -> Self {
        Ingredient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

fn main() {
    let ingredients = vec![
        Ingredient::new(2, 0, -2, 0, 3),
        Ingredient::new(0, 5, -3, 0, 3),
        Ingredient::new(0, 0, 5, -1, 8),
        Ingredient::new(0, -1, 0, 5, 8),
    ];

    const TEASPOONS: i32 = 100;

    let mut best_score = 0;
    let mut best_score_500_calories = 0;
    for first in 0..=TEASPOONS {
        for second in 0..=(TEASPOONS - first) {
            for third in 0..=(TEASPOONS - first - second) {
                let fourth = TEASPOONS - first - second - third;

                let capacity_score = cmp::max(
                    0i32,
                    first * ingredients[0].capacity
                        + second * ingredients[1].capacity
                        + third * ingredients[2].capacity
                        + fourth * ingredients[3].capacity,
                );
                let durability_score = cmp::max(
                    0i32,
                    first * ingredients[0].durability
                        + second * ingredients[1].durability
                        + third * ingredients[2].durability
                        + fourth * ingredients[3].durability,
                );
                let flavor_score = cmp::max(
                    0i32,
                    first * ingredients[0].flavor
                        + second * ingredients[1].flavor
                        + third * ingredients[2].flavor
                        + fourth * ingredients[3].flavor,
                );
                let texture_score = cmp::max(
                    0i32,
                    first * ingredients[0].texture
                        + second * ingredients[1].texture
                        + third * ingredients[2].texture
                        + fourth * ingredients[3].texture,
                );
                let calories = first * ingredients[0].calories
                    + second * ingredients[1].calories
                    + third * ingredients[2].calories
                    + fourth * ingredients[3].calories;

                let score = capacity_score * durability_score * flavor_score * texture_score;

                if score > best_score {
                    best_score = score;
                }
                if calories == 500 && score > best_score_500_calories {
                    best_score_500_calories = score;
                }
            }
        }
    }

    assert_eq!(21_367_368, best_score);
    assert_eq!(1_766_400, best_score_500_calories);
}
