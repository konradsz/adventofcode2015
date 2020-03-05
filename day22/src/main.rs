#[derive(Copy, Clone)]
enum Spell {
    MagicMissle,
    Drain,
    Shield,
    Poison,
    Recharge,
}

#[derive(Copy, Clone)]
struct Wizard {
    hit_points: usize,
    armor: usize,
    mana: usize,
    shield_rounds_left: usize,
    poison_rounds_left: usize,
    recharge_rounds_left: usize,
    mana_used: usize,
}

#[derive(Copy, Clone)]
struct Boss {
    hit_points: usize,
    damage: usize,
}

impl Wizard {
    fn new(hit_points: usize, mana: usize) -> Self {
        Wizard {
            hit_points,
            armor: 0,
            mana,
            shield_rounds_left: 0,
            poison_rounds_left: 0,
            recharge_rounds_left: 0,
            mana_used: 0,
        }
    }

    fn cast_missle(&mut self, boss: &mut Boss) -> bool {
    	if let Some(mana_left) = self.mana.checked_sub(53) {
    		self.mana = mana_left;
            self.mana_used += 53;
            boss.hit_points = boss.hit_points.saturating_sub(4);
    		return true;
    	}

        false
    }

    fn cast_drain(&mut self, boss: &mut Boss) -> bool {
    	if let Some(mana_left) = self.mana.checked_sub(73) {
    		self.mana = mana_left;
            self.mana_used += 73;
	        boss.hit_points = boss.hit_points.saturating_sub(2);
	        self.hit_points += 2;
    		return true;
    	}

        false
    }

    fn cast_shield(&mut self) -> bool {
        if self.shield_rounds_left > 0 {
            return false;
        }

        if let Some(mana_left) = self.mana.checked_sub(113) {
    		self.mana = mana_left;
            self.mana_used += 113;
            self.armor += 7;
        	self.shield_rounds_left = 6;
    		return true;
    	}

        false
    }

    fn cast_poison(&mut self) -> bool {
        if self.poison_rounds_left > 0 {
            return false;
        }

        if let Some(mana_left) = self.mana.checked_sub(173) {
    		self.mana = mana_left;
            self.mana_used += 173;
        	self.poison_rounds_left = 6;
    		return true;
    	}

        false
    }

    fn cast_recharge(&mut self) -> bool {
        if self.recharge_rounds_left > 0 {
            return false;
        }

        if let Some(mana_left) = self.mana.checked_sub(229) {
    		self.mana = mana_left;
            self.mana_used += 229;
        	self.recharge_rounds_left = 5;
    		return true;
    	}

        false
    }

    fn is_shield_active(&self) -> bool {
        self.shield_rounds_left > 0
    }

    fn is_poison_active(&self) -> bool {
        self.poison_rounds_left > 0
    }

    fn is_recharge_active(&self) -> bool {
        self.recharge_rounds_left > 0
    }
}

impl Boss {
    fn new(hit_points: usize, damage: usize) -> Self {
        Boss { hit_points, damage }
    }
}

const HARD_MODE: bool = true;

fn simulate_round(wizard: &mut Wizard, boss: &mut Boss, spell: Spell, best_case: &mut usize) {
    if HARD_MODE {
        wizard.hit_points = wizard.hit_points.saturating_sub(1);
        if wizard.hit_points == 0 {
            return;
        }
    }

    if wizard.mana_used > *best_case {
        return;
    }

    let mut wizard_shield = 0;
    if wizard.is_shield_active() {
        wizard.shield_rounds_left -= 1;
        wizard_shield = 7;
    }

    if wizard.is_poison_active() {
        wizard.poison_rounds_left -= 1;
        boss.hit_points = boss.hit_points.saturating_sub(3);
    }

    if wizard.is_recharge_active() {
        wizard.recharge_rounds_left -= 1;
        wizard.mana += 101;
    }

    match spell {
        Spell::MagicMissle => {
            if !wizard.cast_missle(boss) {
                return;
            }
        },
        Spell::Drain => {
            if !wizard.cast_drain(boss) {
                return;
            }
        },
        Spell::Shield => {
            if !wizard.cast_shield() {
                return;
            }
        },
        Spell::Poison => {
            if !wizard.cast_poison() {
                return;
            }
        },
        Spell::Recharge => {
            if !wizard.cast_recharge() {
                return;
            }
        },
    }

    if wizard.is_shield_active() {
        wizard.shield_rounds_left -= 1;
        wizard_shield = 7;
    }

    if wizard.is_poison_active() {
        wizard.poison_rounds_left -= 1;
        boss.hit_points = boss.hit_points.saturating_sub(3);
    }

    if wizard.is_recharge_active() {
        wizard.recharge_rounds_left -= 1;
        wizard.mana += 101;
    }

    if boss.hit_points == 0 {
        if wizard.mana_used < *best_case {
            *best_case = wizard.mana_used;
        }
        return;
    }

    if HARD_MODE {
        wizard.hit_points = wizard.hit_points.saturating_sub(1);
    }

    wizard.hit_points = wizard.hit_points.saturating_sub(boss.damage - wizard_shield);
    if wizard.hit_points == 0 {
        return;
    }

    for spell in &[
        Spell::MagicMissle,
        Spell::Drain,
        Spell::Shield,
        Spell::Poison,
        Spell::Recharge,
    ] {
        simulate_round(&mut wizard.clone(), &mut boss.clone(), *spell, best_case);
    }
}

fn main() {
    let wizard = Wizard::new(50, 500);
    let boss = Boss::new(55, 8);

    let mut best_case = std::usize::MAX;
    for spell in &[
        Spell::MagicMissle,
        Spell::Drain,
        Spell::Shield,
        Spell::Poison,
        Spell::Recharge,
    ] {
        simulate_round(&mut wizard.clone(), &mut boss.clone(), *spell, &mut best_case);
    }

    println!("{}", best_case);
}
