use crate::schema::promotions;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Clone)]
#[table_name = "promotions"]
pub struct Promotion {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub active: bool,
    pub return_type: String,
    pub return_value: f64,
    pub type_: String,
    pub organization_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PromotionType {
    Discount,
    Coupon,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PromotionReturn {
    Percentage(f64),
    Fixed(f64)
}

impl Promotion {
    pub fn get_type(&self) -> PromotionType {
        match self.type_.as_ref() {
            "discount" => PromotionType::Discount,
            "coupon" => PromotionType::Coupon,
            _ => unreachable!("Invalid promotion type. data corrupted?")
        }
    }

    pub fn get_return(&self) -> PromotionReturn {
        match self.return_type.as_ref(){
            "percentage" => PromotionReturn::Percentage(self.return_value),
            "fixed" => PromotionReturn::Fixed(self.return_value),
            _ => unreachable!("Invalid promotion type. data corrupted?")
        }
    }
}

impl PartialEq for Promotion {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

