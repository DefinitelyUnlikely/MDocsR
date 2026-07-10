use crate::common::error::Error;
use sqlx::PgPool;

pub trait SnapshotRepository: Send + Sync {}
