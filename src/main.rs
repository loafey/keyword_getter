use anyhow::{Error, Result};
use std::{
    collections::BTreeMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn main() -> Result<()> {
    let (path, args) = get_args().ok_or(Error::msg("Did not get correct arguments!"))?;
    let f = File::open(path)?;
    let reader = BufReader::new(f).lines();
    let mut res = args
        .iter()
        .cloned()
        .map(|s| (s, 0))
        .collect::<BTreeMap<_, _>>();
    // using BTreeMap so that output is in the same order each time
    for line in reader {
        let line = line?;
        for a in &args {
            if line.contains(a) {
                if let Some(r) = res.get_mut(a) {
                    *r += 1;
                }
            }
        }
    }
    for (arg, amount) in res {
        println!("{arg}: {amount}");
    }
    Ok(())
}

fn get_args() -> Option<(PathBuf, Vec<String>)> {
    let mut args = env::args().skip(1);
    let path = args.next()?;
    let rest = args.collect::<Vec<_>>();

    if !rest.is_empty() {
        Some((PathBuf::from(path), rest))
    } else {
        None
    }
}
