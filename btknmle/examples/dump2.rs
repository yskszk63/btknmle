use tokio::prelude::*;

use btknmle;
use btknmle_input::event as event;
use btknmle_input::LibinputStream;

#[tokio::main(single_thread)]
async fn main() -> Result<(), failure::Error> {
    use event::pointer::PointerEvent;
    use event::Event;

    let mut kbstat = btknmle::kbstat::KbStat::new();
    let mut mousestat = btknmle::mousestat::MouseStat::new();

    let mut stream = LibinputStream::new_from_udev("seat0")?;
    while let Some(evt) = stream.next().await {
        match evt? {
            Event::Keyboard(kbd) => {
                kbstat.recv(&kbd);
                println!("{:?}", kbstat.to_bytes());
            }
            Event::Pointer(PointerEvent::Motion(motion)) => {
                mousestat.recv_motion(&motion);
                println!("{:?}", mousestat.to_bytes());
            }
            Event::Pointer(PointerEvent::Button(button)) => {
                mousestat.recv_button(&button);
                println!("{:?}", mousestat.to_bytes());
            }
            Event::Pointer(PointerEvent::Axis(axis)) => {
                mousestat.recv_axis(&axis);
                println!("{:?}", mousestat.to_bytes());
            }
            x => println!("{:?}", x),
        }
    }
    Ok(())
}
