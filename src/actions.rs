pub fn hello_action() -> &'static str {
    "hello"
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn hello_test() {
        assert_eq!(hello_action(), "hello")
    }
}
