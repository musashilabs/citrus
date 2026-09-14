pub mod config;
mod error;
pub mod types;
pub mod utils;
pub use utils::*;

pub mod cli;

#[cfg(test)]
mod tests {
    // use super::*;
    //
    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
