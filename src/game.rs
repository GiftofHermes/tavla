use crate::Board;
use crate::Error;
use crate::Player;
use crate::Point;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Die(u8);

impl Die {
    fn new(n: u8) -> Result<Self, Error> {
        if (1..=6).contains(&n) {
            Ok(Die(n))
        } else {
            Err(Error::DieOutOfBounds)
        }
    }

    fn roll() -> Self {
        Die::new(rand::thread_rng().gen_range(1..=6))
            .expect("range is between 1 and 6 so should not return error")
    }
}

#[derive(Debug, Clone, Copy)]
struct Dice(Die, Die);

impl Dice {
    fn roll() -> Self {
        Dice(Die::roll(), Die::roll())
    }

    fn values(&self) -> [u8; 2] {
        [self.0 .0, self.1 .0]
    }
}

#[derive(Default)]
pub enum GameStatus {
    #[default]
    InProgress,
    WhiteWin,
    BlackWin,
}

pub struct Game {
    board: Board,
    player_turn: Player,
    dice: Option<Dice>,
    status: GameStatus,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move {
    from: usize,
    to: usize,
}
impl Move {
    fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::default(),
            player_turn: Player::White,
            dice: None,
            status: GameStatus::default(),
        }
    }

    pub fn roll_dice(&mut self) {
        self.dice = Some(Dice::roll());
    }

    pub fn valid_movements(&self) -> Vec<Move> {
        let mut movements = Vec::new();
        if let Some(dice) = self.dice {
            let dice_values = dice.values();
            for from in 0..24 {
                if let Point::Occupied(player, _) = self.board.points[from] {
                    if player == self.player_turn {
                        for &die_value in &dice_values {
                            let to = match self.player_turn {
                                Player::White => from + die_value as usize,
                                Player::Black => from.saturating_sub(die_value as usize),
                            };

                            let movement = Move::new(from, to);
                            if to < 24 && self.is_valid_movement(movement) {
                                movements.push(Move { from, to });
                            }
                        }
                    }
                }
            }
        }
        movements
    }

    fn is_valid_movement(&self, movement: Move) -> bool {
        let to = movement.to;
        match self.board.points[to] {
            Point::Empty => true,
            Point::Occupied(player, count) => player == self.player_turn || count == 1,
        }
    }

    pub fn make_movement(&mut self, movement: Move) -> Result<(), Error> {
        if !self.valid_movements().contains(&movement) {
            return Err(Error::InvalidMove);
        }

        let from_point = self.board.points[movement.from];
        let to_point = self.board.points[movement.to];

        match (from_point, to_point) {
            (Point::Occupied(player, count), _) if player == self.player_turn => {
                // Removement piece from the 'from' point
                self.board.points[movement.from] = if count > 1 {
                    Point::Occupied(player, count - 1)
                } else {
                    Point::Empty
                };

                // Add piece to the 'to' point
                match to_point {
                    Point::Empty => {
                        self.board.points[movement.to] = Point::Occupied(self.player_turn, 1);
                    }
                    Point::Occupied(existing_player, existing_count) => {
                        if existing_player == self.player_turn {
                            self.board.points[movement.to] =
                                Point::Occupied(self.player_turn, existing_count + 1);
                        } else {
                            assert_eq!(
                                existing_count, 1,
                                "Invalid state: opponent has more than one piece on point"
                            );
                            self.board.points[movement.to] = Point::Occupied(self.player_turn, 1);
                            self.board.bar[existing_player as usize] += 1;
                        }
                    }
                }

                Ok(())
            }
            _ => Err(Error::InvalidMove),
        }
    }

    pub fn switch_turn(&mut self) {
        self.player_turn = match self.player_turn {
            Player::White => Player::Black,
            Player::Black => Player::White,
        };
        self.dice = None;
    }

    pub fn check_win(&self) -> bool {
        self.board
            .points
            .iter()
            .filter(|x| match x {
                Point::Empty => false,
                Point::Occupied(player, _) => {
                    if *player == self.player_turn {
                        true
                    } else {
                        false
                    }
                }
            })
            .count()
            == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = Game::new();
        assert!(matches!(game.player_turn, Player::White));
        assert!(game.dice.is_none());
    }

    #[test]
    fn test_roll_dice() {
        let mut game = Game::new();
        game.roll_dice();
        assert!(game.dice.is_some());
    }

    #[test]
    fn test_valid_movements() {
        let mut game = Game::new();
        game.roll_dice();
        let movements = game.valid_movements();
        assert!(!movements.is_empty());
    }

    #[test]
    fn test_make_movement() {
        let mut game = Game::new();
        game.roll_dice();
        let movements = game.valid_movements();
        if let Some(movement) = movements.first() {
            assert!(game.make_movement(*movement).is_ok());
        }
    }

    #[test]
    fn test_switch_turn() {
        let mut game = Game::new();
        assert!(matches!(game.player_turn, Player::White));
        game.switch_turn();
        assert!(matches!(game.player_turn, Player::Black));
    }
}
