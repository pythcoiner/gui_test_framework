

#[derive(Debug)]
pub enum Error {
    FailFindWindow(String),
    FailFetchPosition,
    FailCapture(String),
    #[allow(unused)]
    DetectItem,
}
