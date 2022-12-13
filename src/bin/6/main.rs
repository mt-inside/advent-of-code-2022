use clap::Parser;
use std::collections::HashMap;
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
    let mut count = HashMap::with_capacity(26); // try to save a couple allocations

    println!("{} bytes, span {}", buf.len(), args.span);

    /* I believe this algorithm is pretty optimal.
     * - it might be quicker to leave 0s in the map and count the non-0 keys, than to spam the allocator like this algo does
     * - except it probably doesn't after it's been warmed - the keys probably just become None?
     * For a simpler but probably slower one, see the git history
     */
    let mut loc = 0;
    for i in 0..buf.len() {
        let cur = *count.get(&buf[i]).unwrap_or(&0) + 1;
        count.insert(buf[i], cur);
        if i >= args.span {
            let old = *count.get(&buf[i - args.span]).unwrap() - 1;
            if old == 0 {
                count.remove(&buf[i - args.span]);
            } else {
                count.insert(buf[i - args.span], old);
            }
        }
        // println!("{}", std::str::from_utf8(&buf[0..=i])?);
        // for (k, v) in &count {
        //     print!("{}: {},  ", *k as char, v);
        // }
        // println!();
        if count.len() == args.span {
            loc = i + 1;
            break;
        }
    }

    println!("At: {}", loc);

    Ok(())
}
