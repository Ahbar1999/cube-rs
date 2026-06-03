pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
pub mod task;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task;


    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
