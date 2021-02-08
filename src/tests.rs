#[test]
async fn yay() {
    let client = crate::client::MbtcClient::new(String::from("a"), String::from("b"));
    client.query_mbtc_api().await?;
    assert_eq!(2 + 2, 4);
}
