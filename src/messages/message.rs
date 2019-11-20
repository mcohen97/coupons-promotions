use crate::models::{Promotion};
use crate::messages::MessageSender;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Message {
    PromotionCreated(Promotion),
    PromotionUpdate(Promotion),
    PromotionDeleted(Id),
    PromotionEvaluated(EvaluationResult),
}

impl Message {
    pub fn send(self, sender: MessageSender) {
        sender.send(self)
    }

    pub fn get_routing_key(&self) -> &'static str {
        match self {
            Message::PromotionCreated(_) => "promotion.created",
            Message::PromotionUpdate(_) => "promotion.updated",
            Message::PromotionDeleted(_) => "promotion.deleted",
            Message::PromotionEvaluated(_) => "promotion.evaluated",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EvaluationResult {
    pub promotion_id: i32,
    pub organization_id: i32,
    pub result: EvaluationInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demographic_data: Option<DemographyData>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct EvaluationInfo {
    pub applicable: bool,
    pub response_time: u128,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_discounted: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DemographyData {
    pub city: String,
    pub country: String,
    pub birth_date: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Id {
    pub id: i32
}

impl From<i32> for Id {
    fn from(id: i32) -> Self {
        Id { id }
    }
}