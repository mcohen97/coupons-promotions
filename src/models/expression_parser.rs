use std::collections::HashMap;
use evalexpr::{build_operator_tree, Node, Context, HashMapContext, Value};
use crate::server::{ApiResult, ApiError};

pub struct PromotionExpression {
    ast: Node
}

impl PromotionExpression {
    pub fn parse(code: &str) -> ApiResult<Self> {
        debug_assert_eq!(code, code.to_lowercase(), "Code must be lowercase"); // Code must be all lower case
        let expr = Self::transform_expression(code);
        let ast = build_operator_tree(&expr)?;

        Ok(PromotionExpression { ast })
    }

    fn transform_expression(raw: &str) -> String {
        raw.trim_start_matches("if")
            .trim_end_matches("then apply_discount")
            .replace("and", "&&")
            .replace("or", "||")
    }

    pub fn evaluate(&self, attributes: HashMap<String, f64>) -> ApiResult<bool>  {

        let mut context = HashMapContext::new();
        for (key, val) in attributes {
            context.set_value(key, val.into())?;
        }
        context.set_value("valid_coupon_code".into(), Value::Boolean(true))?;
        context.set_value("valid_transaction".into(), Value::Boolean(true))?;

        let result = self.ast.eval_boolean_with_context_mut(&mut context)?;
        Ok(result)
    }

    fn validate_attributes(&self, attributes: &HashMap<String, f64>) -> ApiResult<()>  {
        if let Some(&product_size) = attributes.get("product_size") {
            if product_size < 0.0 { return Err(ApiError::from("Product size must be positive"))}
        }

        if let Some(&total) = attributes.get("total") {
            if total < 0.0 { return Err(ApiError::from("Total must be positive"))}
        }

        if let Some(&quantity) = attributes.get("quantity") {
            if quantity < 0.0 { return Err(ApiError::from("Quantity must be positive"))}
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
    }

    #[test]
    fn valid_discount_code() {
        let exp = discount_expression();
        let attributes = hashmap!["total" => 10000.0, "quantity" => 4.0];
        let result = exp.evaluate(attributes).unwrap();
        assert!(result)
    }

    #[test]
    fn valid_discount_code_or() {
        let exp = discount_expression();
        let attributes = hashmap!["total" => 250.0, "quantity" => 6.0];
        let result = exp.evaluate(attributes).unwrap();
        assert!(result)
    }

    #[test]
    fn invalid_discount_code() {
        let exp = discount_expression();
        let attributes = hashmap!["total" => 150.0, "quantity" => 2.0];
        let result = exp.evaluate(attributes).unwrap();
        assert!(!result)
    }

    #[test]
    fn valid_coupon_code() {
        let exp = coupon_expression();
        let attributes = hashmap!["total" => 101.0, "products_size" => 3.0];
        let result = exp.evaluate(attributes).unwrap();
        assert!(result)
    }

    #[test]
    fn invalid_coupon_code() {
        let exp = coupon_expression();
        let attributes = hashmap!["total" => 150.0, "products_size" => 1.0];
        let result = exp.evaluate(attributes).unwrap();
        assert!(!result)
    }

    fn coupon_expression() -> PromotionExpression {
        let code = "if valid_coupon_code
                          and total > 100
                          and products_size >=2
                          then apply_discount";
        PromotionExpression::parse(code).unwrap()
    }

    fn discount_expression() -> PromotionExpression {
        let code = "if valid_transaction
                          and (
                            (total <= 1000 and quantity >= 5)
                            or
                            total > 1000)
                          then apply_discount";
        PromotionExpression::parse(code).unwrap()
    }
}