//! Describe the crate a bit.
//! It is an absolutely super fancy adder. **Must have** dependency!
//! 
//! This is *markdown*. Use Links and lists and all that stuff... 
//! * [crates.io](https://crates.io/)
//! * [mlc](https://github.com/becheran/mlc)

/// Add two numbers
///
/// So exiting! You can add two `u32` numbers!1!! 
/// 
/// # Examples
///
/// ```
/// let answer = doctest::add(1, 2);
/// assert_eq!(answer, 3);
/// ```
pub fn add(n1: u32, n2: u32) -> u32 {
    n1 + n2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_two_success() {
        assert_eq!(add(4, 2), 6);
    }

    #[test]
    #[should_panic]
    fn add_two_overflow() {
        add(u32::MAX, 1);
    }
}
