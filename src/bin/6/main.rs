use clap::Parser;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    span: usize,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut f = File::open("data/6.txt")?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    let mut window = vec![buf[0]; args.span]; // ugly, but stops a,b,c,0 causing it to pop

    println!("{} bytes", buf.len());

    let mut loc = 0;
    for i in 0..buf.len() {
        window[i % args.span] = buf[i];
        println!("Window: {:?}", std::str::from_utf8(&window).unwrap());
        let check: HashSet<_, RandomState> = HashSet::from_iter(window.iter());
        if check.len() == args.span {
            loc = i + 1;
            break;
        }
    }

    println!("At: {}", loc);

    Ok(())
}
