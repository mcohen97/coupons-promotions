#[allow(unused_variables)]
use std::collections::HashMap;
use evalexpr::{build_operator_tree, Node, Context, HashMapContext};
use crate::server::ApiResult;

pub struct PromotionExpression {
    ast: Node
}

impl PromotionExpression {
    pub fn parse(code: &str) -> ApiResult<Self> {
        debug_assert_eq!(code, code.to_lowercase()); // Code must be all lower case
        let mut expr = code.trim_start_matches("if");
        expr = expr.trim_end_matches("then apply_discount");
        let e = expr.replace("\n", "");
        let ast = build_operator_tree(&e)?;

        Ok(PromotionExpression { ast })
    }

    pub fn evaluate<T>(&self, attributes: HashMap<T, f64>) -> ApiResult<bool> where T: Into<String> {
        let mut context = HashMapContext::new();
        for (key, val) in attributes {
            context.set_value(key.into(), val.into())?;
        }
        let result = self.ast.eval_boolean_with_context(&context)?;

        Ok(result)
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
        let exp = coupon_expression();
        let attributes = hashmap!["total" => 500.0, "quantity" => 4.0];
        let result = exp.evaluate(attributes).unwrap();
        assert!(result)
    }

    #[test]
    fn valid_discount_code_or() {
        let exp = coupon_expression();
        let attributes = hashmap!["total" => 10000.0, "quantity" => 4.0];
        let result = exp.evaluate(attributes).unwrap();
        assert!(result)
    }

    #[test]
    fn invalid_discount_code() {
        let exp = coupon_expression();
        let attributes = hashmap!["total" => 150.0, "products_size" => 10.0];
        let result = exp.evaluate(attributes).unwrap();
        assert!(result)
    }

    #[test]
    fn valid_coupon_code() {
        let exp = coupon_expression();
        let attributes = hashmap!["total" => 50.0, "products_size" => 1.0];
        let result = exp.evaluate(attributes).unwrap();
        assert!(result)
    }

    #[test]
    fn invalid_coupon_code() {
        let exp = coupon_expression();
        let attributes = hashmap!["total" => 150.0, "products_size" => 1.0];
        let result = exp.evaluate(attributes).unwrap();
        assert!(result)
    }

    fn coupon_expression() -> PromotionExpression {
        let code = "if valid_coupon_code
                          and total > 100
                          and products_size >=2
                          then apply_discount";
        PromotionExpression::parse(code).unwrap()
    }

    fn discount_expression() -> PromotionExpression {
        let code = "if valid_transaction (1)
                          or (total <= 1000 AND quantity >= 5) (2)
                          or total > 1000
                          then apply_discount";
        PromotionExpression::parse(code).unwrap()
    }
}