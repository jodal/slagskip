mod game;
mod grid;
mod player;
mod ship;

#[derive(Debug, Eq, PartialEq)]
pub struct New;

#[derive(Debug, Eq, PartialEq)]
pub struct Active;

pub use crate::game::game::{Game, GameResult};
pub use crate::game::grid::{Cell, Fire, Grid, Point};
pub use crate::game::player::Player;
pub use crate::game::ship::{Direction, Ship};
