mod game;
use game::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tavla = Game::new();
        assert!(tavla);
    }
}
