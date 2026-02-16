use tikv_client::RawClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("tikv_client DB _Hello, world!");

    let client = RawClient::new(vec!["127.0.0.1:2379"]).await?;

    let key = "Hello".to_owned();
    let value = "RawKV".to_owned();

    // put
    let result = client.put(key.to_owned(), value.to_owned()).await?;
    assert_eq!(result, ());

    // get
    let result = client.get(key.to_owned()).await?;
    assert_eq!(result.unwrap(), value.as_bytes());

    // delete
    let result = client.delete(key.to_owned()).await?;
    assert_eq!(result, ());

    // get
    let result = client.get(key.to_owned()).await?;
    assert_eq!(result, None);

    // scan
    let limit = 1000;
    client.put("k1".to_owned(), "v1".to_owned()).await?;
    client.put("k2".to_owned(), "v2".to_owned()).await?;
    client.put("k3".to_owned(), "v3".to_owned()).await?;
    client.put("k4".to_owned(), "v4".to_owned()).await?;
    let result = client.scan("k1".to_owned().."k5".to_owned(), limit).await?;
    println!("{:?}", result);
    Ok(())
}
