mod error;
mod game;

pub use error::Error;
pub use game::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _tavla = Game::new();
        assert!(true);
    }
}
