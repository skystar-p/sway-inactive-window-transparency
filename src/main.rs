use tokio_i3ipc::{event::Subscribe, I3};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut conn = I3::connect().await?;

    conn.subscribe([Subscribe::Window]).await?;

    let mut listener = conn.listen();

    while let Some(event) = listener.next().await {
        println!("{:?}", event);
    }

    Ok(())
}
