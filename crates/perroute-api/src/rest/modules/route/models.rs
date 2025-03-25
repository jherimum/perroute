use crate::rest::{
    models::resource::ResourceModel,
    modules::business_unit::models::BusinessUnitPath,
};
use bon::Builder;
use chrono::NaiveDateTime;
use perroute_commons::types::{id::Id, priority::Priority, Configuration};
use perroute_storage::models::route::Route;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::Deref};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RoutePath {
    business_unit_id: String,
    route_id: String,
}

impl RoutePath {
    pub fn business_unit_id(&self) -> Id {
        Id::from(&self.business_unit_id)
    }

    pub fn route_id(&self) -> Id {
        Id::from(&self.route_id)
    }

    pub fn business_unit_path(&self) -> RouteCollectionPath {
        RouteCollectionPath {
            business_unit_id: self.business_unit_id.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct RouteCollectionPath {
    business_unit_id: String,
}

impl RouteCollectionPath {
    pub fn business_unit_id(&self) -> Id {
        Id::from(&self.business_unit_id)
    }

    pub fn business_unit_path(&self) -> BusinessUnitPath {
        BusinessUnitPath::new(&self.business_unit_id)
    }
}

#[derive(Debug, Builder, Deserialize)]
pub struct CreateRouteRequest {
    channel_id: String,
    message_type_id: String,
    configuration: HashMap<String, String>,
    priority: i64,
    enabled: bool,
}

impl CreateRouteRequest {
    pub fn channel_id(&self) -> Id {
        Id::from(&self.channel_id)
    }

    pub fn message_type_id(&self) -> Id {
        Id::from(&self.message_type_id)
    }

    pub fn configuration(&self) -> Configuration {
        Configuration::new(&self.configuration)
    }

    pub fn priority(&self) -> Priority {
        Priority::new(self.priority)
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }
}

#[derive(Debug, Deserialize, Builder)]
pub struct UpdateRouteRequest {
    pub configuration: HashMap<String, String>,
    pub priority: i64,
    pub enabled: bool,
}

impl UpdateRouteRequest {
    pub fn configuration(&self) -> Configuration {
        Configuration::new(&self.configuration)
    }

    pub fn priority(&self) -> Priority {
        Priority::new(self.priority)
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }
}

#[derive(Debug, Serialize)]
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

impl From<Route> for RouteModel {
    fn from(route: Route) -> Self {
        RouteModel {
            id: route.id().to_string(),
            channel_id: route.channel_id().to_string(),
            message_type_id: route.message_type_id().to_string(),
            configuration: route.configuration().deref().clone(),
            priority: **route.priority(),
            enabled: *route.enabled(),
            created_at: **route.created_at(),
            updated_at: **route.updated_at(),
        }
    }
}

impl From<Route> for ResourceModel<RouteModel> {
    fn from(route: Route) -> Self {
        ResourceModel::new(RouteModel::from(route))
    }
}
