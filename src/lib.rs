pub mod grammar;
pub mod interpreter;
mod jump_table;
mod memory;
pub mod parser;
pub mod token;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
