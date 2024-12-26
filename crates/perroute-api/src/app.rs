use crate::rest::{
    error::ApiError,
    modules::{
        business_unit::service::BusinessUnitRestService,
        channel::service::ChannelRestService,
        message_type::service::MessageTypeRestService,
        route::service::RouteRestService, routes,
    },
};
use actix_web::{dev::Server, middleware::Logger, web::Data, App, HttpServer};
use actix_web_validator::{JsonConfig, PathConfig, QueryConfig};
use std::{error::Error, net::TcpListener};

pub struct Application {
    server: Server,
}

impl Application {
    pub fn new<
        RS: BusinessUnitRestService
            + MessageTypeRestService
            + ChannelRestService
            + RouteRestService
            + Clone
            + Send
            + Sync
            + 'static,
    >(
        listener: TcpListener,
        rest_service: RS,
    ) -> Result<Self, Box<dyn Error>> {
        let server = HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .app_data(json_config())
                .app_data(path_config())
                .app_data(query_config())
                .app_data(Data::new(rest_service.clone()))
                .service(routes::<RS>())
        })
        .listen(listener)?
        .run();
        Ok(Self { server })
    }

    pub async fn start(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

fn json_config() -> JsonConfig {
    JsonConfig::default()
        .limit(4096)
        .error_handler(|err, _| actix_web::Error::from(ApiError::from(err)))
}

fn path_config() -> PathConfig {
    PathConfig::default()
        .error_handler(|err, _| actix_web::Error::from(ApiError::from(err)))
}

fn query_config() -> QueryConfig {
    QueryConfig::default()
        .error_handler(|err, _| actix_web::Error::from(ApiError::from(err)))
}
