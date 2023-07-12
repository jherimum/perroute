use std::net::TcpListener;

use perroute_backoffice_api::app::server;
use sqlx::PgPool;
use url::Url;

pub struct TestApp {
    url: Url,
}

impl TestApp {
    pub async fn start(pool: PgPool) -> TestApp {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = Url::parse(&format!("http://{}", listener.local_addr().unwrap())).unwrap();
        let server = server(listener, pool).await.unwrap();
        let app = TestApp { url };
        tokio::spawn(server);
        app
    }

    pub fn path<I: AsRef<str>>(&self, join: impl IntoIterator<Item = I>) -> Url {
        join.into_iter().fold(self.url.clone(), |url, path| {
            url.join(path.as_ref()).unwrap()
        })
    }
}
