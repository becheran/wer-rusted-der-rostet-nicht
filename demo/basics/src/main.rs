fn main() {
    // Define immutable variable i
    let i = 5;
    // This is not allowed: i += 2;
    let j = 10;
    let result = add(i, j);
    println!("result = {}", result);
}

/// Add two numbers and return the result
fn add(n1: u32, n2: u32) -> u32 {
    n1 + n2 // Equal to `return n1 + n2;
}

#[cfg(test)] // Only compile for debug
mod test {
    // Import everything from the parent module
    use super::*;
    use ntest::test_case;

    // One UnitTest test case
    #[test]
    fn add_two_success() {
        assert_eq!(add(4, 2), 6);
    }

    // Show use of imported library macro function
    #[test_case(1, 2, 3)]
    #[test_case(2, 2, 4)]
    fn demo_test_case(i: u32, j: u32, res: u32) {
        assert_eq!(add(i, j), res);
    }
}
