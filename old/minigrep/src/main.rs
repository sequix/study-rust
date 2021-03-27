use minigrep::Config;
use std::env;
use std::process;

fn main() {
    // TODO: &Vec<string> 到  &[String] 的转换是怎么做的？
    // TODO: &String 到 &str 的装换呢？
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("error parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error {}", e);
        process::exit(1);
    }
}

// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents =
//         fs::read_to_string(&config.filename).expect(&format!("reading file {}", &config.filename));

//     println!("with text:\n{}", contents);

//     Ok(())
// }

// struct Config {
//     query: String,
//     filename: String,
// }

// impl Config {
//     pub fn new(args: &[String]) -> Result<Config, &str> {
//         // TODO：&[String] 的函数在哪定义的？
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }

//         let query = args[1].clone();
//         let filename = args[2].clone();

//         Ok(Config { query, filename })
//     }
// }
