#![allow(non_snake_case)]
#![allow(unused_imports)]

use encoding_rs::{self, Encoding};
use core::panic;
use std::fs::File;
use std::path::Path;
use std::{error, io::*};

fn ReadFile(pathText: &str) -> std::fs::File
{
    let s = pathText.to_string();
    let path = Path::new(&s);
    let display = path.display();

    let f = match File::open(&path)
    {
        Err(why) => panic!("couldn't open {}: {}", display,why.to_string()),

        Ok(file) => file,
    };

    return f;
}

fn main()
{
    let app= clap::App::new("NRTCC")
        .version("0.1.0")
        .author("NyanRus <touyu1829@gmail.com>")
        .about("NyanRus Txt FIle Codepage Converter")
        .arg(clap::Arg::with_name("in")
            .help("input txt")
            .short("i")
            .long("input")
            .takes_value(true)
            .required(true)
        )
        .arg(clap::Arg::with_name("out")
            .help("output txt")
            .short("o")
            .long("output")
            .takes_value(true)
            .required(true)
        )
        .arg(clap::Arg::with_name("cp")
            .help("codepage \n https://encoding.spec.whatwg.org/#concept-encoding-get")
            .short("cp")
            .long("codepage")
            .takes_value(true)
            .required(true)
        )
        .get_matches();

    let in_f = app.value_of("in").unwrap();
    let out_f = app.value_of("out").unwrap();
    let cp = app.value_of("cp").unwrap();

    let file = ReadFile(in_f);
    let f = BufReader::new(file);
    let mut outPath = std::fs::File::create(out_f).unwrap();

    for result in f.lines()
    {
        let a = result.unwrap();

        let mut v: Vec<String> = Vec::<String>::with_capacity(1024);
        for i in a.as_str().chars()
        {
            if i.is_ascii()
            {
                let tmp = hex::encode(String::from(i));
                //v.insert(0,tmp);
                v.push(tmp);
            }
            else
            {
                let st = String::from(i);
                let enc = Encoding::for_label(cp.as_bytes());
                if enc == None
                {
                    eprintln!("Can't recognize codepage.\nPlease refer \"https://encoding.spec.whatwg.org/#concept-encoding-get\"");
                    std::process::exit(1);
                }
                let (Cow,_,_) = encoding_rs::Encoding::encode(enc.unwrap(), &st);
                let s = hex::encode(Cow.to_owned());
                let chs = s.chars().collect::<Vec<char>>();
                println!("{}",s);
                /*
                if chs.len() == 4
                {
                    v.insert(0,format!("{}{}{}{}",chs[2],chs[3],chs[0],chs[1]));
                }
                else
                {
                    v.insert(0,s);
                }
                */
                v.push(s);
            }
        }
        let b : String= v.into_iter().collect();

        let c = format!("{}\n",b);
        let d  = c.as_str();
        outPath.write(d.as_bytes()).unwrap();
    }
}