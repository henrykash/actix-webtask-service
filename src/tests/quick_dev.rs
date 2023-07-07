use anyhow::Result;

#[toko::test] 
async fn quick_dev()-> Result<()> {
    let hc = httpc::new_client("http://localhost:8080")?;

    hc.do_get("/hello").await?.print().await?; // returns a response

    Ok(())
}