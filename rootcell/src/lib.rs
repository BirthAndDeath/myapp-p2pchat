#![forbid(unsafe_code)]
#![deny(
    // 代码质量
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    unused_extern_crates,
    // 危险模式
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    // 安全相关
    clippy::as_conversions,
    clippy::cast_ptr_alignment,
    clippy::integer_arithmetic,
    // 性能
    clippy::inefficient_to_string,
    clippy::unnecessary_to_owned,
)]

//! # Chat Root of Trust
//!分布智能需要保证加密和权利义务对等
//! 独立信任根 crate，负责：
//! - 硬件密钥派生
//! - 会话密钥管理
//! - 加密配置存储
//!
//! 需遵循安全保证：(注意当前未实现许多部分)
//! - 零 unsafe 代码
//! - 最小依赖树
//! - 常量时间操作
//!
use std::fmt;

use subtle::ConstantTimeEq;
use zeroize::{Zeroize, ZeroizeOnDrop};
mod cilent;
mod platform;
mod server;
///! 信任根错误类型
#[derive(thiserror::Error, Debug)]
pub enum TrustError {
    ///密钥访问被拒绝
    #[error("AccessDenied")]
    AccessDenied,
    ///硬件安全模块不可用
    #[error("HardwareUnavailable")]
    HardwareUnavailable,
    ///密钥已撤销:{0}
    #[error("KeyRevoked:{0}")]
    KeyRevoked(String),
    ///加密操作失败
    #[error("CryptoFailure")]
    CryptoFailure,
    ///存储错误: {0}
    #[error(" Storage error:{0}")]
    Storage(String),
}
///!安全核心模块
struct SecurityCore {
    //密钥
    key: SecretKey,
    //消息缓存
    message_cache: Vec<u8>,
}
impl SecurityCore {
    fn new() -> Result<Self, TrustError> {
        let key;
        let newkey = SecretKey::generate();
        if let Err(e) = newkey {
            return Err(e);
        } else {
            key = newkey.unwrap();
        }

        Ok(Self {
            key: key,
            message_cache: Vec::new(),
        })
    }
}
///密钥类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum KeyType {
    /// 身份密钥（长期，硬件绑定）
    Identity,
    /// 会话密钥（短期，内存中）
    Session,
    /// 前向保密密钥（一次性，自动轮换）
    PreKey,
    /// 群组密钥（层级派生）
    Group,
}
///密钥数据

/// 32字节对称密钥（AES-256/ChaCha20-Poly1305）
///
/// 安全特性：
/// - 禁止 Clone/Copy（防止意外内存复制）
/// - Drop 时自动清零（ZeroizeOnDrop）
/// - 调试输出掩盖（防止日志泄露）
/// - 常量时间比较（防时序攻击）
#[derive(ZeroizeOnDrop)]
pub struct SecretKey {
    // 私有字段，模块外无法访问
    bytes: [u8; 32],

    // 内存污染标记（检测 use-after-free）
    #[zeroize(skip)]
    version: u64,
}

/// 密钥使用一次的包装器（类似 move 语义）
#[derive(Debug)]
pub struct OneTimeKey(SecretKey);
impl SecretKey {
    /// 生成新密钥（CSPRNG）
    pub fn generate() -> Result<Self, TrustError> {
        let mut bytes = [0u8; 32];
        getrandom::fill(&mut bytes).map_err(|_| TrustError::CryptoFailure)?;

        Ok(Self { bytes, version: 1 })
    }

    /// 从字节数组创建（接管所有权，输入立即清零）
    pub fn from_bytes(mut bytes: [u8; 32]) -> Self {
        let key = Self { bytes, version: 1 };
        bytes.zeroize(); // 清零输入副本
        key
    }

    /// 暴露为切片（仅限内部模块使用）
    ///
    /// # Safety
    /// 返回的切片必须在调用点立即使用，不得存储
    pub(crate) fn expose_secret(&self) -> &[u8; 32] {
        &self.bytes
    }

    /// 派生子密钥（HKDF-SHA256）
    pub fn derive(&self, context: &[u8], subkey_id: u64) -> Result<Self, TrustError> {
        use ring::hkdf::{HKDF_SHA256, Salt};

        let salt = Salt::new(HKDF_SHA256, &self.bytes);
        let prk = salt.extract(&[]);

        let info = [context, &subkey_id.to_be_bytes()].concat();
        let mut okm = [0u8; 32];

        // ring 0.17：expand -> Okm -> fill
        prk.expand(&[&info], HKDF_SHA256)
            .map_err(|_| TrustError::CryptoFailure)?
            .fill(&mut okm)
            .map_err(|_| TrustError::CryptoFailure)?;

        Ok(Self {
            bytes: okm,
            version: 1,
        })
    }

    /// 常量时间相等性比较
    pub fn ct_eq(&self, other: &Self) -> bool {
        self.bytes.ct_eq(&other.bytes).into()
    }

    /// 显式清零（即使还有引用也执行）
    pub fn burn(&mut self) {
        self.bytes.zeroize();
        self.version = 0; // 标记为已销毁
    }

    /// 验证密钥未被污染
    pub fn is_valid(&self) -> bool {
        self.version != 0 && !bool::from(self.bytes.ct_eq(&[0u8; 32]))
    }
}

/// 禁止 Clone（防止隐式内存复制）
impl Clone for SecretKey {
    fn clone(&self) -> Self {
        panic!(
            "SecretKey cannot be cloned. Use key derivation or Arc<Mutex<SecretKey>> if sharing is absolutely necessary"
        );
    }
}

/// 禁止 Debug 泄露内容
impl fmt::Debug for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SecretKey")
            .field("version", &self.version)
            .field("bytes", &"[REDACTED]")
            .finish()
    }
}

/// 禁止 Display
impl fmt::Display for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[REDACTED]")
    }
}

/// 禁止序列化（防止意外写入日志/网络）
impl serde::Serialize for SecretKey {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Err(serde::ser::Error::custom(
            "SecretKey serialization is forbidden",
        ))
    }
}

/// 禁止反序列化（防止从不可信来源恢复）
impl<'de> serde::Deserialize<'de> for SecretKey {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Err(serde::de::Error::custom(
            "SecretKey deserialization is forbidden",
        ))
    }
}

impl OneTimeKey {
    /// 消费密钥，获取内部值（只能调用一次）
    pub fn into_inner(mut self) -> SecretKey {
        std::mem::replace(&mut self.0, SecretKey::generate().expect("CSPRNG failed"))
    }
}

impl Drop for OneTimeKey {
    fn drop(&mut self) {
        self.0.burn();
    }
}
