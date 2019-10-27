use tokio::prelude::*;

use btknmle;
use btknmle_input::event as event;
use btknmle_input::LibinputStream;
use btknmle_input::{ButtonCodes, KeyCodes};

#[tokio::main(single_thread)]
async fn main() -> Result<(), failure::Error> {
    use event::pointer::PointerEvent;
    use event::Event;

    let mut kbstat = btknmle::kbstat::KbStat::new();
    let mut stream = LibinputStream::new_from_udev("seat0")?;
    while let Some(evt) = stream.next().await {
        match evt? {
            Event::Keyboard(kbd) => {
                kbstat.recv(&kbd);
                println!("{:?}", kbstat);
                println!("{:?}", kbstat.to_bytes());
            }
            Event::Pointer(PointerEvent::Motion(motion)) => {
                println!("{} {}", motion.dx(), motion.dy())
            }
            Event::Pointer(PointerEvent::Button(button)) => println!(
                "{:?} 0x{:?}",
                button.button_state(),
                ButtonCodes::from(button.button())
            ),
            Event::Pointer(PointerEvent::Axis(axis)) => {
                use event::pointer::Axis;
                if axis.has_axis(Axis::Horizontal) {
                    println!("0x{:?} H", axis.axis_value(Axis::Horizontal));
                }
                if axis.has_axis(Axis::Vertical) {
                    println!("0x{:?} V", axis.axis_value(Axis::Vertical));
                }
            }
            x => println!("{:?}", x),
        }
    }
    Ok(())
}
