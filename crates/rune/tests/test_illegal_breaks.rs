use rune::EncodeError::*;
use st::unit::Span;

macro_rules! test_encode_err {
    ($pat:pat => $cond:expr, $source:expr) => {{
        let err = rune::compile($source).unwrap_err();

        match err {
            rune::Error::EncodeError($pat) => ($cond),
            _ => {
                panic!("expected error `{}` but was `{:?}`", stringify!($pat), err);
            }
        }
    }};
}

#[test]
fn break_outside_of_loop() {
    test_encode_err! {
        BreakOutsideOfLoop { span } => assert_eq!(span, Span::new(41, 46)),
        r#"
            fn main() {
                break;
            }
        "#
    };
}

#[test]
fn test_break_as_value() {
    test_encode_err! {
        BreakDoesNotProduceValue { span } => assert_eq!(span, Span::new(41, 46)),
        r#"
            fn main() {
                break
            }
        "#
    };
}
