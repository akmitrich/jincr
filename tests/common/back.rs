use testcontainers_modules::testcontainers::runners::AsyncRunner;

pub struct TestBackend {
    #[allow(unused)]
    container: testcontainers_modules::testcontainers::ContainerAsync<
        testcontainers_modules::postgres::Postgres,
    >,
    pub database_url: String,
}

impl TestBackend {
    pub async fn setup() -> TestBackend {
        let (database_url, container) = setup_test_database().await;
        TestBackend {
            container,
            database_url,
        }
    }

    pub fn create_pg(&self) -> jincr::backend::Pg {
        jincr::backend::Pg::connect_to(&self.database_url).unwrap()
    }
}
async fn setup_test_database() -> (
    String,
    testcontainers_modules::testcontainers::ContainerAsync<
        testcontainers_modules::postgres::Postgres,
    >,
) {
    println!("starting container");
    let container = testcontainers_modules::postgres::Postgres::default()
        .start()
        .await
        .unwrap();
    let host_ip = container.get_host().await.unwrap();
    let host_port = container.get_host_port_ipv4(5432).await.unwrap();
    let database_url = format!("postgres://postgres:postgres@{host_ip}:{host_port}/postgres");
    println!("now connect to {database_url}");
    (database_url, container)
}
