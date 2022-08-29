use std::path::Path;
use std::fs::OpenOptions;
use std::io::{Cursor, Read};
use byteorder::{NativeEndian, ReadBytesExt};
use std::{thread, time};

mod  details;

fn main(){
    let system_details = details::SystemDetails::details();
    //println!("{:?}",system_details.event_path);
    let path = Path::new(system_details.event_path.as_str());
    let mut file_options = OpenOptions::new();
    file_options.read(true);
    file_options.write(false);
    
    

    // Read the event file
    loop {
    let mut event_file = file_options.open(path).unwrap();
    let mut packet = [0u8; 24];
    event_file.read_exact(&mut packet).unwrap();
    let mut rdr = Cursor::new(packet);
    let tv_sec  = rdr.read_u64::<NativeEndian>().unwrap();
    let tv_usec = rdr.read_u64::<NativeEndian>().unwrap();
    let evtype  = rdr.read_u16::<NativeEndian>().unwrap();
    let code    = rdr.read_u16::<NativeEndian>().unwrap();
    let value   = rdr.read_i32::<NativeEndian>().unwrap();

    println!("{} ", value);
    let ten_millis = time::Duration::from_millis(100);
    thread::sleep(ten_millis);
    }
   
}








