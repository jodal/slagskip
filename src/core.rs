mod game;
mod grid;
mod player;
mod ship;

#[derive(Debug, Eq, PartialEq)]
pub struct New;

#[derive(Debug, Eq, PartialEq)]
pub struct Active;

pub use crate::core::game::{Game, GameResult};
pub use crate::core::grid::{Cell, Fire, Grid, Point};
pub use crate::core::player::Player;
pub use crate::core::ship::{Direction, Ship};
