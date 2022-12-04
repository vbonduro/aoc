pub mod input;

#[cfg(test)]
mod tests {
    use crate::input;
    #[test]
    fn it_works() {
        println!("{}", input::get(2021, 1).unwrap());
    }
}
