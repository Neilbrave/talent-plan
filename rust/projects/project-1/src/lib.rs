
//#![deny(missing_docs)]

//!kvs是一个内存存储的数据库，支持set，get，remove操作。不具有持久性。

mod kv_engine;

pub use kv_engine::KvStore;

