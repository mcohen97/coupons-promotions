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

pub enum PromotionType {
    Discount,
    Coupon,
}

pub enum PromotionReturn {
    Percentage(f64),
    Fixed(f64)
}

impl Promotion {
    pub fn new(
        name: String,
        code: String,
        active: bool,
        p_return: PromotionReturn,
        p_type: PromotionType,
        organization_id: i32, ) -> Self {
        let (return_type, return_value) = match p_return {
            PromotionReturn::Percentage(val) => ("percentage".into(), val),
            PromotionReturn::Fixed(val) => ("fixed".into(), val)
        };
        let type_ = match p_type {
            PromotionType::Coupon => "coupon".into(),
            PromotionType::Discount => "discount".into()
        };

        Promotion {id: 0, name, code, active, return_value, return_type, type_, organization_id}
    }

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

#[derive(Debug, Serialize, Deserialize)]
pub enum EvaluationResult {
    PromotionDoesntApply,
    PromotionApplies { discount_type: String },
}

