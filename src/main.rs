use std::path::Path;
use std::fs::OpenOptions;
use std::io::{Cursor, Read};
use byteorder::{NativeEndian, ReadBytesExt};
use std::{thread, time};

mod  details;

fn main(){
    let system_details = details::SystemDetails::details();
    //Open the keyboard event file 
    let path = Path::new(system_details.event_path.as_str());
    let mut file_options = OpenOptions::new();
    //Open the file in read-only mode
    file_options.read(true);
    file_options.write(false);
    file_options.create(false);

    // Read the event file
    // Perform the read operation in a loop so it wont stop
    loop {
        let mut event_file = file_options.open(path).unwrap();
        let mut packet = [0u8; 24];
        event_file.read_exact(&mut packet).unwrap();
        //Use byteorder lib for reading file as binary
        //from the 24byte stream we only require last 8 bytes that represent the keycode
        //Drop the unwanted bytes
        let mut rdr = Cursor::new(packet);
        let tv_sec  = rdr.read_u64::<NativeEndian>().unwrap();
        let tv_usec = rdr.read_u64::<NativeEndian>().unwrap();
        let evtype  = rdr.read_u16::<NativeEndian>().unwrap();
        let code    = rdr.read_u16::<NativeEndian>().unwrap();
        let value   = rdr.read_i32::<NativeEndian>().unwrap();
        drop(tv_sec);
        drop(tv_usec);
        drop(evtype);
        drop(code);
        //print value of keycode
        println!("{} ", value as u8);
        //sleep for 0.1 seconds to avoid printing too fast
        let ten_millis = time::Duration::from_millis(100);
        thread::sleep(ten_millis);
        
    }
   
}
