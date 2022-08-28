#![allow(non_snake_case)]

use std::io::BufRead;
use clap::Parser;

#[derive(Debug)]
struct CustomError(String);

//Custom parser struct that derives from claps parser
#[derive(Parser)]
struct Cli {
    pattern: String,

    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let args = Cli::parse();

    let file = std::fs::File::open(&args.path).map_err(|err| format!("Error! Could not read {}, {}", args.path.display(), err))?;
    let reader = std::io::BufReader::new(file);

    let mut numLinesContainingMsg = 0;
    for line in reader.lines()
    {
        let line = line.map_err(|err| format!("Error! Could not read line, {}", err))?;

        if line.contains(&args.pattern)
        {
            numLinesContainingMsg += 1;
            println!("{}", line);
        }
    }

    println!("{} values counted", numLinesContainingMsg);

    Ok(())
}
