use thiserror::Error;


#[derive(Error, Debug)]
pub enum PXQError {
    #[error("请求数据错误")]
    ReqwestError,

    #[error("解析数据错误")]
    ParseDataError,

    #[error("文件访问错误")]
    FileAccessError,

    #[error("获取用户信息失败")]
    GetUserProfileError,
}

impl serde::Serialize for PXQError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
