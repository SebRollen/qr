use image::Luma;
use qrcode::QrCode;
use std::env::{args, temp_dir};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        println!("USAGE: qr https://www.example.com");
        std::process::exit(1);
    }
    let code = QrCode::new(args[1].as_bytes()).unwrap();
    let image = code.render::<Luma<u8>>().build();
    let mut out = temp_dir();
    out.push("qr.png");
    image.save(out.clone()).unwrap();
    std::process::Command::new("open").arg(out).spawn().unwrap();
}
