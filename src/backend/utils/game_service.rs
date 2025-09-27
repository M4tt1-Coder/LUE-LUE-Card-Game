use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha8Rng,
};

use crate::backend::enums::card_types::CardType;

/// Randomly generates a new card type like 'King' or 'Queen'.
///
/// It uses CSPRNG function to ensure best practice for random-generated output.
pub fn select_new_card_to_be_played() -> CardType {
    let mut rng = ChaCha8Rng::from_seed(Default::default());
    let num: usize = (rng.next_u32() % CardType::number_of_values() as u32) as usize;
    return CardType::from_usize(num);
}
