#[macro_export]
macro_rules! assert_that {
    ($this:expr, $that:expr $(,)?) => {
        assert_that!($this, $that);
    };
    ($this:expr, $that:expr, $($arg:tt)+) => {{
        match (&$this, &$that) {
            (this, that) => {
                if !(*this == *that) {
                    ::std::panic!(
                        "\n  Expected: {:?}\n  Was:      {:?}\n  {}",
                        that,
                        this,
                        ::std::format_args!($($arg)+)
                    );
                }
            }
        }
    }};
}
#[macro_export]
macro_rules! make_test {
    ($name:ident: $input:expr => $exp:tt < $stdin:expr) => {
        #[test]
        fn $name() {
            use ::std::io::Cursor;
            let msg = ::std::format!(
                "==> Test input was: '{}' => '{}' < \"{:?}\"",
                $input,
                ::std::stringify!($exp),
                $stdin
            );
            $crate::assert_that!(
                ::stack_machine::run_with_input($input, &mut Cursor::new(String::from($stdin)))
                    .expect(&msg)
                    .pop(),
                ::std::option::Option::<_>::Some(::stack_machine::Value::from($exp)),
                "{}",
                msg
            )
        }
    };
    ($name:ident: $input:expr => todo! $msg:tt < $stdin:expr) => {
        $crate::make_test!($name: $input => todo! $msg);
    };
    ($name:ident: $input:expr => todo! $msg:expr) => {
        #[test]
        fn $name() {
            ::std::todo!("{}: todo: {}", stringify!($name), $msg);
        }
    };
    ($name:ident: $input:expr => $exp:expr) => {
        make_test!($name: $input => @[$exp]);
    };
    ($name:ident: $input:expr => @[ $($exp:expr),* $(,)? ]) => {
        #[test]
        fn $name() {
            let msg = format!(
                "==> Test input was: '{}' => [{}]",
                $input,
                stringify!($($exp),*)
            );
            $crate::assert_that!(
                ::stack_machine::run($input).expect(&msg),
                vec![$(::stack_machine::Value::from($exp)),*],
                "{}",
                msg
            )
        }
    };
}

#[macro_export]
macro_rules! v {
    ($v:expr) => {
        ::stack_machine::Value::from($v)
    };
}
