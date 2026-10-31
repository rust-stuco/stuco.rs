#![forbid(unsafe_code)]
#![allow(dead_code, unused_variables)]

use crate::bomb::Bomb;

/// Prevents a [`Bomb`] from being dropped.
///
/// # Examples
///
/// ```
/// use bomblab::{Bomb, defuser::defuse};
///
/// let bomb = Bomb;
/// defuse(bomb);
/// ```
pub fn defuse(bomb: Bomb) {
    // TODO(phase 3): Save the world
}
