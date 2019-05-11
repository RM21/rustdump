extern crate byteorder;
extern crate hex;

use std::env;
use std::fs::File;
use std::io::Cursor;
use std::time::Duration;
use std::io::prelude::*;
use hex::decode;
use byteorder::{LittleEndian, BigEndian, ReadBytesExt};

fn main() {
	// const F_HEADER_LENGTH: u32 = 24;

    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

	println!("Read file {}", filename);
	
	let mut f = File::open(filename).expect("file not found");

	let mut header = [0; 4];

	f.read(&mut header).expect("Dun goofed.");

	// println!("b {}", header);
	println!("Magic Number: {:x?}", Cursor::new(&header).read_u32::<LittleEndian>().unwrap());

	let mut header = [0; 2];

	f.read(&mut header).expect("Dun goofed.");

	// println!("b {}", header);
	println!("Major version: {:x?}", Cursor::new(&header).read_u16::<LittleEndian>().unwrap());

	let mut header = [0; 2];

	f.read(&mut header).expect("Dun goofed.");

	// println!("b {}", header);
	println!("Minor version: {:x?}", Cursor::new(&header).read_u16::<LittleEndian>().unwrap());

	let mut header = [0; 4];

	f.read(&mut header).expect("Dun goofed.");

	// println!("b {}", header);
	println!("Accuracy: {:x?}", Cursor::new(&header).read_u32::<LittleEndian>().unwrap());

	let mut header = [0; 4];

	f.read(&mut header).expect("Dun goofed.");

	// println!("b {}", header);
	println!("Timezone offset: {:x?}", Cursor::new(&header).read_u32::<LittleEndian>().unwrap());

	let mut header = [0; 4];

	f.read(&mut header).expect("Dun goofed.");

	// println!("b {}", header);
	println!("Snapshot length: {:x?}", Cursor::new(&header).read_u32::<LittleEndian>().unwrap());

	let mut header = [0; 4];

	f.read(&mut header).expect("Dun goofed.");

	// println!("b {}", header);
	println!("Pcap linktype: {:x?}", Cursor::new(&header).read_u32::<LittleEndian>().unwrap());

	let mut header = [0; 4];

	f.read(&mut header).expect("Dun goofed.");

	// println!("b {}", header);
	println!("First timestamp: {:x?}", Duration::new(Cursor::new(&header).read_u32::<LittleEndian>().unwrap().into(), 0));

	let mut header = [0; 4];

	f.read(&mut header).expect("Dun goofed.");

	// println!("b {}", header);
	println!("Microsecond timestamp: {:x?}", Duration::new(Cursor::new(&header).read_u32::<LittleEndian>().unwrap().into(), 0));

	let mut header = [0; 4];

	f.read(&mut header).expect("Dun goofed.");

	// println!("b {}", header);
	println!("Number of octets: {}", Cursor::new(&header).read_u32::<LittleEndian>().unwrap());

	let mut header = [0; 4];

	f.read(&mut header).expect("Dun goofed.");

	// println!("b {}", header);
	println!("Actual length: {}", Cursor::new(&header).read_u32::<LittleEndian>().unwrap());

	let mut data = [0; 8];

	f.read(&mut data).expect("Dun goofed.");

	println!("Actual length: {:x?}", data);

	for (_i, elem) in data.iter_mut().enumerate() {
	    println!("{:?}", *elem as char);
	}
}

// fn as_u32_be(array: &[u8; 4]) -> u32 {
//     ((array[3] as u32) << 24) |
//     ((array[2] as u32) << 16) |
//     ((array[1] as u32) <<  8) |
//     ((array[0] as u32) <<  0)
// }

	// println!("Running tcpdump...");

	// let output = Command::new("/usr/sbin/tcpdump")
	// 		.arg("-nn")
	// 		// .arg("src 192.168.43.188")
	// 		.arg("-G 5")
	// 		.arg("-W 1")
	// 		.arg("-w 1")
	// 		.output()
	// 		.expect("tcpdump failed to start");

	// println!("status: {}", output.status);
	// println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
	// println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

	// assert!(output.status.success());	

// fn main() {

//     let args: Vec<String> = env::args().collect();

//     let filename = &args[1];

// 	println!("Read file {}", filename);

// 	unsafe {
// 		let mut f = File::open(filename).expect("file not found");

// 	    let mut a = [0; 4];

//    	    f.read(&mut a).expect("Something");

// 	    let b = mem::transmute::<[u8; 4], u32>(a);

// 	    println!("{}", b);

// 	    let c: u32 = mem::transmute(a);

// 	    println!("{}", c);

// 	    println!("b {:x?}", b); 
// 	    println!("c {:x?}", c); 
// 	    println!("a {:x?}", as_u32_be(&a));
// 	    println!("d {:x?}", Cursor::new(&a.to_vec()).read_u32::<LittleEndian>().unwrap());
// 	}
// }