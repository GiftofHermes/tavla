use crate::Player;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Point {
    Empty,
    Occupied(Player, u8),
}

pub struct Board {
    pub points: [Point; 24],
    pub bar: [u8; 2],
    pub off: [u8; 2],
}

impl Default for Board {
    fn default() -> Self {
        let mut points = [Point::Empty; 24];
        points[0] = Point::Occupied(Player::Black, 2);
        points[5] = Point::Occupied(Player::White, 5);
        points[7] = Point::Occupied(Player::White, 3);
        points[11] = Point::Occupied(Player::Black, 5);
        points[12] = Point::Occupied(Player::White, 5);
        points[16] = Point::Occupied(Player::Black, 3);
        points[18] = Point::Occupied(Player::Black, 5);
        points[23] = Point::Occupied(Player::White, 2);

        Board {
            points,
            bar: [0; 2],
            off: [0; 2],
        }
    }
}
