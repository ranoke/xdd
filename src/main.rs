use std::io::Read;
use clap::Parser;

fn print_hex(chunk: &Vec<u8>, split: usize, upper_case: bool) {
    let chunk_size = chunk.bytes().count();
    let format_fn = if upper_case {
        |b: u8| format!("{:02X}", b)
    } else {
        |b: u8| format!("{:02x}", b)
    };

    for (i, c) in chunk.iter().enumerate() {
        if i != 0 && split != 0 && (i % split == 0) {
            print!(" ");
        }
        print!("{}", format_fn(*c));
    }

    if chunk.bytes().count() >= chunk_size {
        return
    }

    let fill_size = chunk_size - (chunk.bytes().count() % chunk_size);
    let dot: String = String::from("..");
    print!("{}", dot.repeat(fill_size).as_str());
}

fn print_ascii(chunk: &Vec<u8>, split: usize) {
    let size = chunk.bytes().count();
    for (_, c) in chunk.iter().enumerate() {
        if *c == b'\n' {
            let dots: String = String::from("..");
            print!("\n");
            
            if split != 0 {
                let n = size / split;
                for i in 0..n {
                    if i != 0 {
                        print!(" ");
                    }
                    print!("{}", dots.repeat(split));
                } 
            } else {
                print!("{}", dots.repeat(size));
            }

            print!(" | ");
        } else {
            print!("{}", *c as char);
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    filepath: String,

    #[arg(short, long, default_value_t = 16,
        help = "format <cols> octets per line. Default 16 (-i: 12, -ps: 30, -b: 6). Max 256.")]
    cols: u8,

    #[arg(short, default_value_t = false,
        help = "use upper case hex letters. Default is lower case.")]
    uppercase: bool,

    #[arg(short, long, 
        help = "stop after writing <len> octets.")]
    len: Option<usize>,

    #[arg(short, long, default_value_t = 0,
        help = "separate every <split> bytes with space.")]
    split: usize,
}

fn main() {
    let args = CliArgs::parse();
    let filepath = args.filepath;

    let mut f = std::fs::File::open(filepath)
        .expect("Failed to open a specified file");

    let mut buf: Vec<u8> = vec![0; args.cols as usize]; 
    loop {
        let s = f.read(&mut buf).expect("Failed to read file");
        if s == 0 {
            break;
        }
        print_hex(&buf, args.split, args.uppercase);
        print!(" | ");

        print_ascii(&buf, args.split);

        print!("\n");
    }
}
