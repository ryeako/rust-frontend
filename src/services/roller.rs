use rand::prelude::*;

#[derive(Default)]
pub struct DiceRolled {
    rolled_dice: Vec<i32>,
    total: i32
}

impl DiceRolled {
    pub fn total(&self) -> i32 {
        self.total
    }
    pub fn rolled_dice(&self) -> Vec<i32> {
        self.rolled_dice.iter().map(|p| p.clone()).collect()
    }
}

pub fn roller(dice: String) -> Option<DiceRolled> {
    if dice.is_empty() {
        return None;
    }

    if dice.find("d") == None {
        return None;
    }

    let dice_return: Option<DiceRolled>;
    let mut dice_num: i32 = 1;
    let mut dice_type: i32 = 1;

    // split dice and clear blanks
    let dice_split: Vec<&str> = dice.split("d").filter(|&s| s != "").collect();

    if dice_split.len() == 1 {
        dice_type = dice_split[0].parse::<i32>().unwrap();
    }

    if dice_split.len() > 1 {
        dice_num = dice_split[0].parse::<i32>().unwrap();
        dice_type = dice_split[1].parse::<i32>().unwrap();
    }

    let mut sum: i32 = 0;
    let mut every_roll: Vec<i32> = Vec::new();
    let mut rng = rand::thread_rng();
    for _i in 0..dice_num {
        let roll = rng.gen_range(1..=dice_type);
        every_roll.push(roll);
        sum += roll; 
    }
    dice_return = Some(DiceRolled {
        total: sum,
        rolled_dice: every_roll
    });

    dice_return
}