use crate::rest::{routes::routes, services::RestService};
use actix_web::{dev::Server, web::Data, App, HttpServer};
use std::{error::Error, net::TcpListener};

pub struct Application {
    server: Server,
}

impl Application {
    pub fn new<RS: RestService + Clone + Send + Sync + 'static>(
        listener: TcpListener,
        rest_service: RS,
    ) -> Result<Self, Box<dyn Error>> {
        let server = HttpServer::new(move || {
            App::new()
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
