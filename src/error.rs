use new_string_template::error::TemplateError;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    LengthLimit,
    NoValidName,
    StringTemplate(String),
}

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LengthLimit => f.write_str("The length limit cannot be smaller than the input username"),
            Self::NoValidName => f.write_str("There are no templates that can fit in the character limit using specified username"),
            Self::StringTemplate(inner) => f.write_str(inner.to_string().as_str())
        }
    }
}

impl From<TemplateError> for Error {
    fn from(e: TemplateError) -> Error {
        Error::StringTemplate(format!("{}", e))
    }
}
