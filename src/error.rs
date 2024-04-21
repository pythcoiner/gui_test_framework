use rusty_tesseract::TessError;

#[derive(Debug)]
pub enum Error {
    FailFindWindow(String),
    FailFetchPosition,
    FailCapture(String),
    Tesseract(TessError),
}

impl From<TessError> for Error {
    fn from(err: TessError) -> Self {
        Error::Tesseract(err)
    }
}