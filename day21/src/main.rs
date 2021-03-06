use itertools::Itertools;

struct HitPoints(usize);
struct Damage(usize);
struct Armor(usize);
struct Gold(usize);

struct Warrior {
    hit_points: HitPoints,
    damage: Damage,
    armor: Armor,
}

struct Item {
    damage: Damage,
    armor: Armor,
    cost: Gold,
}

impl Warrior {
    fn new(hit_points: HitPoints, damage: Damage, armor: Armor) -> Self {
        Warrior {
            hit_points,
            damage,
            armor,
        }
    }

    fn equip_item(&mut self, item: &Item) {
        self.damage.0 += item.damage.0;
        self.armor.0 += item.armor.0;
    }

    fn get_damage_against(&self, rival: &Warrior) -> usize {
        std::cmp::max(1, self.damage.0.saturating_sub(rival.armor.0))
    }
}

impl Item {
    fn weapon(damage: Damage, cost: Gold) -> Self {
        Item {
            damage,
            armor: Armor(0),
            cost,
        }
    }

    fn armor(armor: Armor, cost: Gold) -> Self {
        Item {
            damage: Damage(0),
            armor,
            cost,
        }
    }

    fn ring(damage: Damage, armor: Armor, cost: Gold) -> Self {
        Item {
            damage,
            armor,
            cost,
        }
    }
}

fn check_if_player_wins(player: &Warrior, boss: &Warrior) -> bool {
    let player_dmg_per_round = player.get_damage_against(&boss);
    let boss_dmg_per_round = boss.get_damage_against(&player);

    let rounds_to_kill_boss = (boss.hit_points.0 as f64 / player_dmg_per_round as f64).ceil();
    let rounds_to_kill_player = (player.hit_points.0 as f64 / boss_dmg_per_round as f64).ceil();

    rounds_to_kill_player > rounds_to_kill_boss
}

fn main() {
    let weapons = vec![
        Item::weapon(Damage(4), Gold(8)),  // Dagger
        Item::weapon(Damage(5), Gold(10)), // Shortsword
        Item::weapon(Damage(6), Gold(25)), // Warhammer
        Item::weapon(Damage(7), Gold(40)), // Longsword
        Item::weapon(Damage(8), Gold(74)), // Greataxe
    ];

    let armors = vec![
        Item::armor(Armor(0), Gold(0)),   // No armor
        Item::armor(Armor(1), Gold(13)),  // Leather
        Item::armor(Armor(2), Gold(31)),  // Chainmail
        Item::armor(Armor(3), Gold(53)),  // Splintmail
        Item::armor(Armor(4), Gold(75)),  // Bandedmail
        Item::armor(Armor(5), Gold(102)), // Platemail
    ];

    let rings = vec![
        Item::ring(Damage(0), Armor(0), Gold(0)),   // No ring
        Item::ring(Damage(0), Armor(0), Gold(0)),   // No ring
        Item::ring(Damage(1), Armor(0), Gold(25)),  // Damage +1
        Item::ring(Damage(2), Armor(0), Gold(50)),  // Damage +2
        Item::ring(Damage(3), Armor(0), Gold(100)), // Damage +3
        Item::ring(Damage(0), Armor(1), Gold(20)),  // Defense +1
        Item::ring(Damage(0), Armor(2), Gold(40)),  // Defense +2
        Item::ring(Damage(0), Armor(3), Gold(80)),  // Defense +3
    ];

    let mut best_buy = std::usize::MAX;
    let mut worst_buy = std::usize::MIN;

    for weapon in weapons.iter() {
        for armor in armors.iter() {
            for rings in rings.iter().combinations(2) {
                let mut player = Warrior::new(HitPoints(100), Damage(0), Armor(0));
                let boss = Warrior::new(HitPoints(109), Damage(8), Armor(2));
                player.equip_item(weapon);
                player.equip_item(armor);
                player.equip_item(rings[0]);
                player.equip_item(rings[1]);

                let cost = weapon.cost.0 + armor.cost.0 + rings[0].cost.0 + rings[1].cost.0;
                if check_if_player_wins(&player, &boss) {
                    if cost < best_buy {
                        best_buy = cost;
                    }
                } else if cost > worst_buy {
                    worst_buy = cost;
                }
            }
        }
    }

    assert_eq!(111, best_buy);
    assert_eq!(188, worst_buy);
}
