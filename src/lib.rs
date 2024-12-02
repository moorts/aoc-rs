#![feature(binary_heap_into_iter_sorted)]
pub mod day1;
pub mod day2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
