#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InnerSdkError {
    pub source: String,
    pub code: usize,
    pub message: String,
}

error_chain! {

    types {
        TonError, TonErrorKind, TonResultExt, TonResult;
    }

    foreign_links {
        Io(std::io::Error);
        SerdeJson(serde_json::Error);
        TryFromSliceError(std::array::TryFromSliceError);
        ParseIntError(std::num::ParseIntError);
        FromHexError(hex::FromHexError);
        Base64DecodeError(base64::DecodeError);
    }

    errors {
        NotFound {
            description("Requested item not found")
        }
        InvalidOperation(msg: String) {
             description("Invalid operation"),
             display("Invalid operation: {}", msg)
        }
        InvalidData(msg: String) {
            description("Invalid data"),
            display("Invalid data: {}", msg)
        }
        InvalidArg(msg: String) {
            description("Invalid argument"),
            display("Invalid argument: {}", msg)
        }
        InvalidFunctionParams(func: String, inner: String){
            description("Invalid function parameters"),
            display("Can not serialize params for {}. Error {}", func, inner)
        }
        InvalidFunctionResult(func: String, result: String, inner: String){
            description("Invalid function parameters"),
            display("Can not deserialize result for {}\nresult JSON: {}\ninner error {}", func, result, inner)
        }
        InvalidFunctionError(func: String, error: String, inner: String){
            description("Invalid function parameters"),
            display("Can not deserialize error for {}\nerror JSON: {}\ninner error {}", func, error, inner)
        }
        InternalError(msg: String) {
            description("Internal error"),
            display("Internal error: {}", msg)
        }
        InnerSdkError(inner: InnerSdkError) {
            description("Inner SDK error"),
            display("Inner SDK error.\nsource: {}\ncode: {}\n message: {}", inner.source, inner.code, inner.message)
        }
    }
}