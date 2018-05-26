extern crate clap;

use std::env;
use std::process::exit;
use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::time::{SystemTime, UNIX_EPOCH};
use clap::{Arg, App};

const VERSION: &str = "3.0";
const BASE: usize = 36;
const NUMERALS: [char; BASE] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
                                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
                                'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];


fn strip_zeroes(string: String) -> String {
    let mut zeroes_count: usize = 0;

    for c in string.chars() {
        if c == NUMERALS[0] {
            zeroes_count += 1;
        } else {
            break;
        }
    }
    return string.chars().skip(zeroes_count).collect();
}


fn int2base(num: usize) -> String {
    if num == 0 {
        return NUMERALS[0].to_string();
    } else {
        return int2base(num / BASE) + &NUMERALS[num % BASE].to_string();
    }
}

fn versions() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let _raw: String = int2base(since_the_epoch.as_secs() as usize);
    // return _raw.trim_left_matches('0');
    return strip_zeroes(_raw);
}

fn get_fix(mut value: String, env_name: String) -> std::io::Result<String> {
    if value.is_empty() {
        value = match env::var(&env_name) {
            Ok(val) => val,
            Err(_e) => String::from(""),
        };
    }

    if value.starts_with("@") {
        let path: String = value.chars().skip(1).collect();
        value = String::new();
        let mut file = File::open(path)?;
        file.read_to_string(&mut value)?;
    }

    return Ok(value);
}

fn write_file(path: &String, version: &String) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(version.as_bytes())?;
    Ok(())
}


fn file_read_error(e: std::io::Error) -> String {
    println!("error reading file: {:?}", e);
    exit(1);
}


fn file_write_error(e: std::io::Error) {
    println!("error writing file: {:?}", e);
    exit(1);
}


fn main() {
    let matches = App::new("Version number generator.")
                            .version(VERSION)
                            .author("Alexander Zelenyak <zzz.sochi@gmail.com>")
                            .about("Generate versions from unix time.")
                            .arg(Arg::with_name("version")
                                .long("version")
                                .help("print version :-)"))
                            .arg(Arg::with_name("silent")
                                .long("silent")
                                .short("s")
                                .help("do not print version"))
                            .arg(Arg::with_name("prefix")
                                .long("prefix")
                                .env("VERSIONS_PREFIX")
                                .value_name("STRING")
                                .takes_value(true)
                                .help("prefix for version"))
                            .arg(Arg::with_name("suffix")
                                .long("suffix")
                                .env("VERSIONS_SUFFIX")
                                .value_name("STRING")
                                .takes_value(true)
                                .help("suffix for version"))
                            .arg(Arg::with_name("file")
                                .value_name("FILE")
                                .takes_value(true)
                                .help("file for save version"))
                            .get_matches();

    if matches.is_present("version") {
        println!("Versions version {} :-)", VERSION);
        return
    }

    let vers: String = versions();
    let prefix = match get_fix(matches.value_of("prefix").unwrap_or("").to_string(), String::from("PRUGIN_PREFIX")) {
        Ok(val) => val,
        Err(e) => file_read_error(e),
    };
    let suffix = match get_fix(matches.value_of("suffix").unwrap_or("").to_string(), String::from("PRUGIN_SUFFIX")) {
        Ok(val) => val,
        Err(e) => file_read_error(e),
    };

    let result = format!("{}{}{}", prefix, vers, suffix);

    if matches.is_present("file") {
        let path = matches.value_of("file").unwrap().to_string();
        let write_result = write_file(&path, &result);
        match write_result {
            Ok(v) => v,
            Err(e) => file_write_error(e),
        }
    }

    if !matches.is_present("silent") {
        println!("{}", &result);
    }
}
