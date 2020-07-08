use blurz::BluetoothSession;
use blurz::BluetoothDevice;

fn main() {
    let path = "/org/bluez/hci0/dev_70_26_05_10_48_04";

    let session = BluetoothSession::create_session(None).unwrap();
    let device = BluetoothDevice::new(&session, path.to_string());

    match device.connect(2_000) {
        Ok(_) => println!("Connected!"),
        Err(e) => println!("Error {}", e)
    }
}
