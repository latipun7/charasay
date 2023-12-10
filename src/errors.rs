#[derive(Debug)]
enum ReadInputError {
    // Represents an IO error
    IoError(std::io::Error),
    // Represents a UTF-8 decoding error
    Utf8Error(std::str::Utf8Error),
}

impl std::fmt::Display for ReadInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            // Format the error message as "IO error: <error>"
            ReadInputError::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl Error for ReadInputError {}