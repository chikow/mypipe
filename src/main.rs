extern crate clap;
use clap::{Arg, App};
use std::process::Command;

fn main() {
    let matches = App::new("mypipe")
                           .version("1.0")
                           .author("LAOUER Walid <laouer.w@hotmail.com>")
                           .about("MyPipe")
                           .arg(Arg::with_name("input")
                                .short("in")
                                .long("in")
                                .value_name("intput")
                                .help("Use the input")
                                .takes_value(true))
                             .arg(Arg::with_name("output")
                                .short("out")
                                .long("out")
                                .value_name("output")
                                .help("Use the output")
                                .takes_value(true))
                           .get_matches();


    let inpu = matches.value_of("input").unwrap( );
    println!("input value : {}", inpu); 


    let outpu = matches.value_of("output").unwrap( );
    println!("output value : {}", outpu); 


    let inpue = Command::new(inpu.to_string()).output().expect("Error , try another input");


    let inpua = String::from_utf8_lossy(&inpue.stdout).to_string();

    let outpue= Command::new(outpu.to_string()).arg(inpua).output().expect("Error , try another output");


    let result = String::from_utf8_lossy(&outpue.stdout).to_string();

     println!("{}", result );


 }