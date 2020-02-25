use btknmle::input::source::InputSource;
use tokio::stream::StreamExt;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    tokio::task::LocalSet::new().block_on(&mut rt, async {
        let mut source = InputSource::new()?;
        tokio::task::spawn_local(source.runner()?);

        let mut rx = source.subscribe();

        while let Some(evt) = rx.next().await {
            println!("{:?}", evt);
        }
        Ok(())
    })
}
