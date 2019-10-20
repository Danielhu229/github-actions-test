pub fn hello_action() -> &'static str {
    "hello"
}

pub fn problematic_function() -> &'static str {
    "not hello"
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn hello_test() {
        assert_eq!(hello_action(), "hello")
    }

    fn problematic_function_test() {
        assert_eq!(problematic_function(), "hello")
    }
}
