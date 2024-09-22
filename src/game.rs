use crate::Board;
use crate::Error;
use crate::Player;
use crate::Point;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Die(u8);

impl TryFrom<u8> for Die {
    type Error = crate::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Into<u8> for Die {
    fn into(self) -> u8 {
        self.0
    }
}

impl Into<usize> for Die {
    fn into(self) -> usize {
        self.0.into()
    }
}

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
    die: Die,
}
impl Move {
    fn new(from: usize, die: Die) -> Self {
        Self { from, die }
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

    fn can_collect(&self) -> bool {
        let player = self.player_turn;

        match player {
            Player::White => {}
            Player::Black => {}
        }
    }

    pub fn valid_movements(&self) -> Vec<Move> {
        let mut movements = Vec::new();
        if let Some(dice) = self.dice {
            let dice_values = dice.values();
            let point_on_bar = self.board.bar[self.player_turn as usize];

            for from in 0..24 {
                if let Point::Occupied(player, _) = self.board.points[from] {
                    if player == self.player_turn {
                        for &die_value in &dice_values {
                            let movement = Move::new(from, Die::try_from(die_value).expect("Because we are creating a die from a die value this should always be a valid operation"));
                            if self.is_valid_movement(&movement) {
                                movements.push(movement);
                            }
                        }
                    }
                }
            }
        }
        movements
    }

    fn is_valid_movement(&self, movement: &Move) -> bool {
        let from = movement.from;
        let die_value: usize = movement.die.into();
        let to: usize = if self.board.bar[self.player_turn as usize] != 0 {
            match self.player_turn {
                Player::White => die_value - 1,
                Player::Black => 24 - die_value,
            }
        } else {
            match self.player_turn {
                Player::White => from + die_value,
                Player::Black => from.saturating_sub(die_value),
            }
        };
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

                let status = match (self.check_win(), player) {
                    (true, Player::White) => GameStatus::WhiteWin,
                    (true, Player::Black) => GameStatus::BlackWin,
                    (false, _) => GameStatus::InProgress,
                };
                self.status = status;

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

    fn check_win(&self) -> bool {
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
