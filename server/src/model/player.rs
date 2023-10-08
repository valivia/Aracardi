use std::ops::RangeBounds;

use super::sub_stage::SubStage;

#[derive(Debug, Clone)]
pub struct Player {
    /// cuid
    pub id: String,
    pub name: String,
    pub avatar: String,
    pub host: bool,
    pub score: i32,
}