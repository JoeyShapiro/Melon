use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::env;
use std::fs::File;
use std::fs;
// extern crate image;
// extern crate num_complex;

fn file2vec(filename: &String) -> Vec<u8>{
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    println!("file is {} Bytes", metadata.len());
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer // return buffer
}

// fn arr2grey(vector: Vec<u8>) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> {
//     let imgx = 244;
//     let imgy = 244;

//     let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

//     for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
//         let cur = vector[x as usize];
//         let r = (cur as f32) as u8;
//         let b = (cur as f32) as u8;
//         let g = (cur as f32) as u8;
//         *pixel = image::Rgb([r, g, b]);
//     }

//     imgbuf
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let file: &String;
    if args.len() > 1 {
        file = &args[1];
    } else {
        println!("ERROR: No file given");
        println!("Example: ./Melon file.ext");
        return;
    }

    // convert to vec
    let byte_vec = file2vec(file);
    println!("{:?}", byte_vec);

    //let file_as_img = arr2grey(byte_vec);
    //file_as_img.save("test.png").unwrap();

    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Connteced to Server");

            let msg = &byte_vec[..];

            stream.write(msg).unwrap();
            println!("sent data awaiting reply...");

            let mut data = [ 0 as u8; 12]; // 12B buffer from server
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    let text = from_utf8(&data).unwrap();
                    println!("{}", text);
                },
                Err(e) => {
                    println!("failed: {}", e);
                }
            }
        },
        Err(e) => {
            println!("failed connect: {}", e);
        }
    }
    println!("Terminated.");
}