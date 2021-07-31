use btknmle_input::LibinputStream;
use futures::StreamExt;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let mut input = LibinputStream::new_from_udev("seat0")?;
    while let Some(event) = input.next().await {
        let event = event?;
        println!("{:?}", event);
    }
    Ok(())
}
