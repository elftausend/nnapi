use nnapi_sys::ResultCode;

pub type Result<T> = core::result::Result<T, ResultCode>;

pub trait IntoResult<T> {
    fn into_result(self) -> Result<T>;
}

impl IntoResult<()> for i32 {
    fn into_result(self) -> Result<()> {
        if self == 0 {
            Ok(())
        } else {
            Err(ResultCode::from(self))
        }
    }
}
