pub mod machine;
pub mod mongo;
pub mod token_blacklist;
pub mod user;

use mongodb::error::{ErrorKind, WriteFailure};

const DUPLICATE_KEY_ERROR_CODE: i32 = 11000;

pub(crate) fn is_duplicate_key_error(err: &mongodb::error::Error) -> bool {
    match err.kind.as_ref() {
        ErrorKind::Write(WriteFailure::WriteError(write_error)) => {
            write_error.code == DUPLICATE_KEY_ERROR_CODE
        }
        _ => false,
    }
}
