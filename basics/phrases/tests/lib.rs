#[cfg(test)]
mod tests {

    extern crate phrases;

    #[test]
    fn english_greeting_correct() {
        assert_eq!("hello", phrases::greetings::english::hello());
    }

    #[test]
    #[should_panic]
    #[ignore]
    fn english_greeting_incorrect() {
        assert_eq!("bonjour", phrases::greetings::english::hello());
    }
}