//! # Dice
//! Dice rolling functionality.
//!
//! This module allows for Arrata-style dice rolling, namely
//! in the format `{quality}{quantity}`, where:
//! - `Quality` is the lower threshold for a successful roll.
//! - `Quantity` is the number of dice (d6's) to roll.
//!
//! Quality is split into three tiers:
//! - `Basic` (>= 4)
//! - `Adept` (>= 3)
//! - `Superb` (>= 2)
//!
//! Usually, we denote a particular Quality by using the capitalized
//! first letter. Thus, a Quality of Basic is refered to in a stat as
//! a `B`.
//!
//! Quantity just defines the number of dice (d6's) we roll; Quantity 5 is
//! 5d6.
//!
//! ## Example
//!
//! Therefore, a stat of Quality Basic and Quantity 4 is refered to as B4.
//!
//! Then, when we roll, we roll 4 dice and any that roll to be 4 or higher
//! are counted and that count is summed into the total successes for the roll.
//!
//! Say we roll and get: `(1, 2, 3, 4)`. In this case, 1 die was >= to 4,
//! so the roll summed to 1 success.
//!
//! ## Advantage
//!
//! Advantage makes it so that if a die rolls the maximum value (6), we
//! add that success to our sum and then re-roll the die. This can continue
//! infinitely until the die comes up short of 6.
//!
//! Advantage can come in more than 1 level, and therefore, if a roll is given
//! at least 2 levels of advantage, we grant additional dice to the roll equal
//! to `(advantage -1)`.
//!
//! Advantage is denoted on a roll with a bang (`!`) prefix, and if advantage
//! is given level 1, the level of advantage follows the bang. So, B4 roll
//! with 3 levels of advantage is written as `!3B4`.
//!
//! ## Disadvantage
//!
//! Disadvantage is the opposite of advantage. If a die rolls the minimum value
//! of 1, the success sum is decremented.
//!
//! As with advantage, disadvantage can come in more than 1 level, and therefore,
//! any levels of disadvantage greater than 1 will remove dice from the roll equal
//! to `(disadvantage - 1)`.
//!
//! We denote disadvantage on a roll with a question mark (`?`) prefix, and if
//! disadvantage is given past level 1, its level follows. Thus, an A3 roll with 2
//! levels of disadvantage is written as `?2A3`.
//!
//! ## Examples w/ Advantage and Disadvantage
//!
//! With advantage (!3B4):
//! `(2, 2, 4, 6) -> (2, 2, 4, 6, 3) -> 2 Successes`
//!
//! With disadvantage (?2A8):
//! `(1, 1, 4, 4, 4, 6, 6) -> 3 Successes`
//!
//! With both advantage and disadvantage (!1?1S10):
//! `(1, 2, 2, 2, 3, 3, 4, 4, 5, 6) -> (1, 2, 2, 2, 3, 3, 4, 4, 5, 6, 6) -> (1, 2, 2, 2, 3, 3, 4, 4, 5, 6, 6, 5) -> 10 Successes`

/// The result of rolling `quantity` dice with a `quality` threshold.
#[derive(Debug, Clone)]
pub struct RollResult {
    /// The number of successes. Can be negative
    /// with disadvantage.
    pub successes: isize,
    /// The number of failures.
    pub failures: usize,
    /// The result of each roll. Will be in the
    /// range 1-6.
    pub results: Vec<u8>,
}

/// Rolls a given stat with advantage and disadvantage.
///
/// # Inputs
///
/// `stat: Stat` - The stat to roll.
///
/// `advantage: usize` - The level of advantage on the roll.
///
/// `disadvantage: usize` - The level of disadvantage on the roll.
///
/// # Outputs
///
/// `DiceResult` - The result of the roll.
#[must_use]
pub fn roll_stat(
    stat: &crate::character::Stat,
    advantage: usize,
    disadvantage: usize,
) -> RollResult {
    let mut quantity = stat.quantity;
    let quality = stat.quality as u8;

    let mut successes = 0;
    let mut failures = 0;

    if advantage > 0 {
        quantity += advantage - 1;
    }

    if disadvantage > 0 {
        // No dice to roll!
        if disadvantage - 1 > quantity {
            return RollResult {
                successes: 0,
                failures: 0,
                results: Vec::new(),
            };
        }
        quantity -= disadvantage - 1;
    }

    let mut results: Vec<u8> = Vec::with_capacity(quantity);

    while quantity > 0 {
        let result: u8 = (rand::random::<u8>() % 6) + 1;
        if advantage > 0 && result == 6 {
            quantity += 1;
        } else if disadvantage > 0 && result == 1 {
            successes -= 1;
        }
        successes += isize::from(result >= quality);
        failures += usize::from(result < quality);
        results.push(result);
        quantity -= 1;
    }

    RollResult {
        successes,
        failures,
        results,
    }
}
