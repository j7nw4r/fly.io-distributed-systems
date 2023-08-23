pub mod error;
pub mod message;
pub mod node;

pub use error::Error;
pub use message::Msg;
pub use message::Type;
pub use node::Node;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
