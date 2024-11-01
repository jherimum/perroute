use bon::Builder;
use chrono::NaiveDateTime;
use serde::Deserialize;
use std::collections::HashMap;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]

pub struct RoutePath {}

#[derive(Debug, Deserialize, Validate)]

pub struct RouteCollectionPath {}

#[derive(Debug, Builder, Deserialize)]

pub struct CreateRouteRequest {
    pub channel_id: String,
    pub message_type_id: String,
    pub configuration: HashMap<String, String>,
    pub priority: i64,
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Builder)]

pub struct UpdateRouteRequest {
    pub configuration: HashMap<String, String>,
    pub priority: i64,
    pub enabled: bool,
}

pub struct RouteModel {
    id: String,
    channel_id: String,
    message_type_id: String,
    configuration: HashMap<String, String>,
    priority: i64,
    enabled: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}
