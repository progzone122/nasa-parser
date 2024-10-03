#[derive(Debug)]
pub enum Errors {
    FailedParseSelector,
    FailedRequest,
    FailedResponseBody,
    FailedParseJson,
    FailedSerializeJson
}

#[derive(Debug)]
pub struct Error {
    code: Errors,
    message: String,
}

impl Error {
    pub fn new<S: Into<String>>(code: Errors, message: S) -> Error {
        Self {
            code,
            message: message.into()
        }
    }
    pub fn info(&self) -> String {
        match &self.code {
            Errors::FailedParseSelector => format!("Selector parsing failed: {}", self.message),
            Errors::FailedRequest => format!("Request failed: {}", self.message),
            Errors::FailedResponseBody => format!("Response body processing failed: {}", self.message),
            Errors::FailedParseJson => format!("Json parse failed: {}", self.message),
            Errors::FailedSerializeJson => format!("Serialize json failed: {}", self.message)
        }
    }
}