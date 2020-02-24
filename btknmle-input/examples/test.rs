use anyhow::Result;
use tokio::stream::StreamExt;
use tokio::time::{self, Duration};

use btknmle_input::LibinputStream;

#[tokio::main]
async fn main() -> Result<()> {
    let mut stream = LibinputStream::new_from_udev("seat0")?;

    let mut p1 = time::delay_for(Duration::from_secs(1));
    let mut p2 = time::delay_for(Duration::from_secs(6));
    let mut p1ok = false;
    let mut p2ok = false;

    loop {
        tokio::select! {
            maybe_input = stream.next() => {
                if let Some(input) = maybe_input {
                    println!("{:?}", input?);
                } else {
                    break
                }
            }
            _ = (&mut p1), if !p1ok => {
                println!("grab");
                stream.grab()?;
                p1ok = true;
            }
            _ = (&mut p2), if !p2ok => {
                println!("p2");
                stream.ungrab()?;
                p2ok = true;
            }
        }
    }

    Ok(())
}
