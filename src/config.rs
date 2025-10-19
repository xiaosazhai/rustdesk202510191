use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};

// 定义默认的中继服务器地址
lazy_static::lazy_static! {
    pub static ref PROD_RENDEZVOUS_SERVER: Arc<RwLock<String>> = Arc::new(RwLock::new("rustdesk.aibaocloud.com".to_owned()));
}

// 定义默认的密钥
pub const RS_PUB_KEY: &str = "Pk3HqWa8J38QY2lJuM8frQhLIoKp9dYmlQz7rUmEWmY=";

// 导出其他可能需要的常量
pub const LINK_HEADLESS_LINUX_SUPPORT: bool = false;