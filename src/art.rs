extern crate getopts;

use getopts::{optopt, optflag, getopts, OptGroup};
use std::os;
use std::io::{BufferedReader,File};

fn print_usage(program: &str, _opts: &[OptGroup]) {
    let msg = getopts::usage("", _opts);
    println!("Usage: {} [command]", program);
    println!("{}", msg);
}

fn do_fetch() {
    println!("fetching");
    let path = Path::new("art.json");
    let mut file = BufferedReader::new(File::open(&path));
    for line in file.lines(){
        println!("{}", line.unwrap());
    }
    
}

fn main() {
    let args: Vec<String> = os::args();

    let program = args[0].clone();

    let opts = [
        optflag("h", "help", "print this help menu")
    ];

    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(program.as_slice(), opts);
        return;
    }

    if !matches.free.is_empty() {
        for x in matches.free.iter() {
            match x.as_slice() {
                "fetch" => do_fetch(),
                _ => ()
            }
        }
    } else {
        print_usage(program.as_slice(), opts);
        return;
    };

    /*do_work(input.as_slice(), output);*/
}

