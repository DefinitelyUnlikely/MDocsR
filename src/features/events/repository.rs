use crate::common::error::Error;
use sqlx::PgPool;

pub trait EventRepository: Send + Sync {}