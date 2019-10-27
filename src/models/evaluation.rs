use std::collections::HashMap;

#[derive(Copy)]
pub struct Evaluation {
    code: String,
    attributes: HashMap<String, f64>
}

impl Evaluation {
    pub fn evaluate(&self) -> bool {
        unimplemented!()
    }
}