pub mod app;
pub mod auth;
pub mod tui;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_test() {
        assert_eq!(2 + 2, 4);
    }
}
