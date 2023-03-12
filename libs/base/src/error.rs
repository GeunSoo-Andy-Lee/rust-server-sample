#[allow(dead_code, clippy::enum_variant_names)]
#[derive(Debug)]
pub enum ErrorCode {
    EndOfStream,
    NotEnoughStream,
    MalformedStream,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Error {
    code: ErrorCode,
    message: String,
}

#[allow(dead_code)]
impl Error {
    pub fn new(code: ErrorCode, message: String) -> Self {
        Error{ code, message }
    }
    pub fn code(&self) -> &ErrorCode {
        &self.code
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    //pub as_err<T>(&self) -> Result<T, Error>::Err {
    //   Result<T, Error>:: Err(self)
    //} 
}

#[macro_export]
macro_rules! make_error {
    ($code: expr, $msg: expr) => {
        Err($crate::error::Error::new($code, $msg))
    };
    ($code: expr) => {
        Err($crate::error::Error::new($code, "".to_string()))
    };
}

