mod distro;

use clap::Parser;
use downloader::Downloader;

#[derive(Debug, Parser)]  // Use the clap parser to parse CLI args
struct Cli {
    distro: String,
    channel: String,
    version: Option<f64>
}

fn main() {
    let args = Cli::parse();

    let channel = args.channel.to_owned();
    
    let mut downloader = Downloader::builder().parallel_requests(1).build().unwrap();

    // check distro
    let distro_tmp = args.distro.to_lowercase();
    let distro = distro_tmp.as_ref();
    
    match distro {
        "ubuntu" => {
            println!("Distro: Ubuntu");
        },
        &_ => {
            println!("REE!");
        }
    }
    
    match channel {
        latest => {
            println!("Downloading 22.10");
            let dl = downloader::Download::new("https://mirror.math.princeton.edu/pub/ubuntu-iso/kinetic/ubuntu-22.10-desktop-amd64.iso");
            let result = downloader.download(&[dl]).unwrap();
            
            for r in result {
        match r {
            Err(e) => println!("Error: {}", e.to_string()),
            Ok(s) => println!("Success: {}", &s),
        };
    }
        }
    }
    
    
}
