#![allow(dead_code, unused_variables, unused_imports)]

use actix_web::HttpRequest;
use crate::server::{ApiResult, ApiError, ServiceFactory};
use crate::services::*;
use actix_web::web::{Json, Data};
use actix_web::{web, HttpResponse};
use std::collections::HashMap;
use http::header;
use crate::models::{Promotion, PromotionRepository, Pool, Connection, Demographics};
use diesel::RunQueryDsl;
use std::error::Error;
use chrono::Utc;
use std::time::SystemTime;
use std::rc::Rc;
use std::borrow::Cow;
use crate::server::model_in::{EvaluationIn, EvaluationOut};
use crate::messages::{DemographyData, Message, EvaluationInfo, MessageSender};
use crate::messages::EvaluationResult as MessageEvalResult;
use std::sync::Arc;


pub struct EvaluationController;

impl EvaluationController {
    pub fn post(id: web::Path<i32>, data: Json<EvaluationIn>, sender: web::Data<MessageSender>, factory: Data<ServiceFactory>, req: HttpRequest) -> ApiResult<HttpResponse> {
        let start = SystemTime::now();
        let services = factory.as_services();
        let Services {evaluation: eval_services, demographic: demo_services, ..} = services?;
        let EvaluationIn { required, attributes, demographic_data } = data.into_inner();
        let id = id.into_inner();

        let eval_result = eval_services.evaluate_promotion(id, required, attributes)?;
        let (demo_response, demo) = demo_services.build_demographics_if_valid(demographic_data);
        let res = match eval_result {
            EvaluationResult::Applies { organization_id, total_discount } => EvaluationOut {
                promotion_id: id,
                organization_id,
                demographic_data: demo_response,
                evaluation_info: EvaluationInfo {
                    applicable: true,
                    total_discounted: Some(total_discount),
                    response_time: start.elapsed().unwrap().as_secs_f64(),
                },
            },
            EvaluationResult::DoesntApply { organization_id } => EvaluationOut {
                promotion_id: id,
                organization_id,
                demographic_data: demo_response,
                evaluation_info: EvaluationInfo {
                    applicable: false,
                    total_discounted: None,
                    response_time: start.elapsed().unwrap().as_secs_f64(),
                },
            }
        };
        Self::publish_message(&res, demo, sender.into_inner());

        Ok(HttpResponse::Ok().json(res))
    }


    fn get_authorization(req: &HttpRequest) -> String {
        req.headers()
            .get(header::AUTHORIZATION)
            .map(header::HeaderValue::to_str)
            .map(|r| match r {
                Ok(value) => value.to_string(),
                Err(err) => err.description().to_string()
            })
            .unwrap_or("".into())
    }

    fn publish_message(eval_result: &EvaluationOut, demo: Option<DemographyData>, sender: Arc<MessageSender>) {
        let message = Message::PromotionEvaluated(MessageEvalResult {
            promotion_id: eval_result.promotion_id,
            organization_id: eval_result.organization_id,
            evaluation_info: eval_result.evaluation_info,
            demographic_data: demo,
        });

        message.send(sender);
    }
}
