use log;
use std::error::Error;
use std::thread;
use std::time::Duration;
use x11rb::connection::Connection;

mod csv;
mod window;

fn main() -> Result<(), Box<dyn Error>> {
    let (conn, screen) = x11rb::connect(None).expect("Failed to connect");
    let root = conn.setup().roots[screen].root;
    let initial_window_class = window::get_active_window_class_and_name(&conn, root);
    match initial_window_class {
        Err(why) => log::warn!("{}", why),
        Ok((iwc, iwn)) => println!("{} {}", iwc, iwn),
    }
    println!("{}", root);

    let records = csv::get_records().expect("could not get records");
    dbg!(records);

    // TODO: event
    loop {
        println!("loop start");
        // let event = conn.wait_for_event()?;

        thread::sleep(Duration::from_secs(5));

        let window_class = window::get_active_window_class_and_name(&conn, root);
        match window_class {
            Err(why) => println!("ónei: {}", why),
            Ok((iwc, iwn)) => println!("window: {} {}", iwc, iwn),
        }
    }
}
