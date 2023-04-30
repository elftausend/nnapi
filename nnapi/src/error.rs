use nnapi_sys::ResultCode;

pub type NnapiResult<T> = Result<T, ResultCode>;

pub trait IntoResult<T> {
    fn into_result(self) -> NnapiResult<T>;
}

impl IntoResult<()> for i32 {
    fn into_result(self) -> NnapiResult<()> {
        if self == 0 {
            Ok(())
        } else {
            Err(ResultCode::from(self))
        }
    }
}
