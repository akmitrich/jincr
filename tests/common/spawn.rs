#[derive(Debug, Clone)]
pub struct TestClient {
    pub client: reqwest::Client,
    port: u16,
}

impl TestClient {
    pub async fn spawn_server() -> TestClient {
        let lst = tokio::net::TcpListener::bind("0.0.0.0:0").await.unwrap();
        let port = lst.local_addr().unwrap().port();
        tokio::spawn(jincr::server::run::with_listener(lst).unwrap());
        let client = reqwest::Client::new();
        Self { client, port }
    }

    pub fn base_url(&self) -> String {
        format!("http://localhost:{}", self.port)
    }
}

impl std::ops::Deref for TestClient {
    type Target = reqwest::Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}
