pub fn is_ascii_number(c: u8) -> bool {
    48 <= c && c <= 57
}

pub fn is_ascii_lowercase(c: u8) -> bool {
    97 <= c && c <= 122
}

pub fn is_ascii_uppercase(c: u8) -> bool {
    65 <= c && c <= 90
}

mod tests {
    use super::*;

    #[test]
    fn test_is_ascii_uppercase() {
        assert!(!is_ascii_uppercase(b'a'));
        assert!(!is_ascii_uppercase(b'd'));
        assert!(!is_ascii_uppercase(b'z'));
        assert!(is_ascii_uppercase(b'A'));
        assert!(is_ascii_uppercase(b'T'));
        assert!(is_ascii_uppercase(b'Z'));
    }

    #[test]
    fn test_is_ascii_lowercase() {
        assert!(is_ascii_lowercase(b'a'));
        assert!(is_ascii_lowercase(b'd'));
        assert!(is_ascii_lowercase(b'z'));
        assert!(!is_ascii_lowercase(b'A'));
        assert!(!is_ascii_lowercase(b'T'));
        assert!(!is_ascii_lowercase(b'Z'));
    }


    #[test]
    fn test_is_ascii_number() {
        assert!(is_ascii_number(b'0'));
        assert!(is_ascii_number(b'5'));
        assert!(is_ascii_number(b'9'));
        assert!(!is_ascii_number(b'a'));
        assert!(!is_ascii_number(b'Z'));
        assert!(!is_ascii_number(b'y'));
    }
}
