mod common;
use crate::common::TestApp;
use perroute_backoffice_api::api::{
    models::business_unit::BusinessUnitResource,
    response::{CollectionResourceModel, SingleResourceModel},
};
use perroute_commons::{code, types::id::Id};
use perroute_storage::models::business_unit::BusinessUnitBuilder;
use reqwest::Client;
use sqlx::PgPool;
use std::str::FromStr;

#[sqlx::test(migrator = "perroute_storage::connection_manager::MIGRATOR")]
fn test_channels_empty(pool: PgPool) {
    let app = TestApp::start(pool.clone()).await;
    let client = Client::new();

    let request = client.get(app.path("api/v1/channels"));
    let response = request.try_clone().unwrap().send().await.unwrap();

    assert!(response.status() == 200);
    assert!(response
        .json::<CollectionResourceModel<BusinessUnitResource>>()
        .await
        .unwrap()
        .data
        .is_empty());

    let channel = BusinessUnitBuilder::default()
        .id(Id::default())
        .code(code!("WINE"))
        .name("Wine channel")
        .build()
        .unwrap()
        .save(&pool)
        .await
        .unwrap();

    let response = request.send().await.unwrap();
    assert!(response.status() == 200);

    let data = response
        .json::<CollectionResourceModel<BusinessUnitResource>>()
        .await
        .unwrap()
        .data;

    assert!(data.len() == 1);

    let model: SingleResourceModel<BusinessUnitResource> = SingleResourceModel {
        data: Some(BusinessUnitResource::from(channel)),
        links: Default::default(),
    };

    assert_eq!(data.get(0).unwrap(), &model);
}
