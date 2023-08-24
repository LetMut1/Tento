use std::convert::From;
    use std::error::Error as StdError;
    use std::fmt::Display;
    use std::fmt::Error as FormatError;
    use std::fmt::Formatter;
    use std::io::Error as IOError;
    use toml::de::Error as TomlError;

    #[derive(Debug)]
    pub enum Error {
        LogicError {
            message: &'static str,
        },
        OtherError {
            other_error: OtherError,
        },
    }

    impl Display for Error {
        fn fmt<'a, 'b>(
            &'a self,
            formatter: &'b mut Formatter<'_>,
        ) -> Result<(), FormatError> {
            match *self {
                Self::LogicError {
                    ref message,
                } => {
                    return write!(
                        formatter,
                        "Error, logic: {}.",
                        message
                    );
                }
                Self::OtherError {
                    ref other_error,
                } => {
                    return write!(
                        formatter,
                        "Error, other: {}.",
                        other_error.get_message()
                    );
                }
            }
        }
    }

    impl StdError for Error {}

    impl From<IOError> for Error {
        fn from(value: IOError) -> Self {
            return Self::OtherError {
                other_error: OtherError::new(&value),
            };
        }
    }

    impl From<TomlError> for Error {
        fn from(value: TomlError) -> Self {
            return Self::OtherError {
                other_error: OtherError::new(&value),
            };
        }
    }

    #[derive(Debug)]
    pub struct OtherError {
        message: String,
    }

    impl OtherError {
        pub fn new<E>(error: E) -> Self
        where
            E: StdError,
        {
            return Self {
                message: format!(
                    "{}",
                    error
                ),
            };
        }

        pub fn get_message<'a>(&'a self) -> &'a str {
            return self.message.as_str();
        }
    }

    impl Display for OtherError {
        fn fmt<'a, 'b>(
            &'a self,
            _: &'b mut Formatter<'_>,
        ) -> Result<(), FormatError> {
            return Ok(());
        }
    }