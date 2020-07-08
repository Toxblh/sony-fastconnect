use blurz::BluetoothDevice;
use blurz::BluetoothSession;
use clap::{App, Arg};

fn main() {
    let matches = App::new("sony-fastconnect")
        .version("1.0")
        .author("Anton Palgunov <toxblh@gmail.com>")
        .about("Fast connect Sony headphones")
        .arg(
            Arg::new("disconnect")
                .about("For fast disconnect")
                .short('d')
        )
        .get_matches();

    let path = "/org/bluez/hci0/dev_70_26_05_10_48_04";

    let session = BluetoothSession::create_session(None).unwrap();
    let device = BluetoothDevice::new(&session, path.to_string());

    if matches.is_present("disconnect") {
        match device.disconnect() {
            Ok(_) => println!("Disconnected!"),
            Err(e) => println!("Error {}", e),
        }
    } else {
        match device.connect(5_000) {
            Ok(_) => println!("Connected!"),
            Err(e) => println!("Error {}", e),
        }
    }
}
