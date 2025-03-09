use anyhow::Result;

#[tokio::test]
async fn test_quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http:localhost:8080")?;

    hc.do_get("/upload").await?.print().await?;
    println!("Working");


    Ok(())
}