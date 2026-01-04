use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use std::str::FromStr;


#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum BetStatus {
    Pending,
    Successful,
    Failed,
}

impl std::fmt::Display for BetStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetStatus::Pending => write!(f, "pending"),
            BetStatus::Successful => write!(f, "successful"),
            BetStatus::Failed => write!(f, "failed"),
        }
    }
}

impl FromStr for BetStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(BetStatus::Pending),
            "successful" => Ok(BetStatus::Successful),
            "failed" => Ok(BetStatus::Failed),
            _ => Err(format!("Invalid bet status: {}", s)),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateBetStatusRequest {
    pub status: BetStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BrokerEvent {
    #[serde(rename = "account_created")]
    AccountCreated { account: Account },
    
    #[serde(rename = "account_updated")]
    AccountUpdated { account: Account },
    
    #[serde(rename = "account_deleted")]
    AccountDeleted { id: i64 },
    
    #[serde(rename = "batch_created")]
    BatchCreated { batch: BatchResponse },
    
    #[serde(rename = "batch_completed")]
    BatchCompleted { 
        id: i64, 
        account_id: i64,
    },
    
    #[serde(rename = "bet_status_updated")]
    BetStatusUpdated { bet: Bet },
    
    #[serde(rename = "batch_bets_updated")]
    BatchBetsUpdated { 
        batch_id: i64,
        account_id: i64,
        bets: Vec<Bet>,
    },
}

impl BrokerEvent {
    // Helper to extract event name for SSE
    pub fn event_name(&self) -> &str {
        match self {
            Self::AccountCreated { .. } => "account_created",
            Self::AccountUpdated { .. } => "account_updated",
            Self::AccountDeleted { .. } => "account_deleted",
            Self::BatchCreated { .. } => "batch_created",
            Self::BatchCompleted { .. } => "batch_completed",
            Self::BetStatusUpdated { .. } => "bet_status_updated",
            Self::BatchBetsUpdated { .. } => "batch_bets_updated",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub hostname: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
    pub name: String,
    pub hostname: String,
}

#[derive(Deserialize)]
pub struct BetUpdateRequest {
    pub pid: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Batch {
    pub id: i64,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub meta: JsonValue,
    pub account_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBatchRequest {
    pub meta: JsonValue,
    pub bets: Vec<CreateBetRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBetRequest {
    pub id: i64,
    pub selection: String,
    pub stake: f64,
    pub cost: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Bet {
    pub pid: i64,
    pub id: i64,
    pub selection: String,
    pub stake: f64,
    pub cost: f64,
    pub status: String,
    pub batch_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchResponse {
    pub id: i64,
    pub completed: bool,
    pub created_at: String,
    pub updated_at: String,
    pub meta: JsonValue,
    pub account_id: i64,
    pub bets: Vec<Bet>,
}
