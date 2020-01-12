use futures::stream::StreamExt as _;

use btknmle_input::LibinputStream;
use btknmle_input::{ButtonCodes, KeyCodes};

#[tokio::main(basic_scheduler)]
async fn main() -> Result<(), failure::Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    use input::event::pointer::PointerEvent;
    use input::Event;

    let mut stream = LibinputStream::new_from_udev("seat0", false)?;
    while let Some(evt) = stream.next().await {
        match evt? {
            Event::Keyboard(kbd) => {
                use input::event::keyboard::KeyboardEventTrait as _;
                println!("{:?} {:?}", kbd.key_state(), KeyCodes::from(kbd.key()),);
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
                use input::event::pointer::Axis;
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
