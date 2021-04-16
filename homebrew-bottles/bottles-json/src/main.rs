#![feature(try_trait)]

use serde::Deserialize;

#[derive(Deserialize)]
struct Versions {
    stable: Option<String>,
    bottle: bool,
}

#[derive(Deserialize)]
struct Bottle {
    stable: Option<BottleStable>,
}

#[derive(Deserialize)]
struct BottleStable {
    rebuild: u64,
    files: serde_json::Value,
}

#[derive(Deserialize)]
struct Formula {
    name: String,
    versions: Versions,
    bottle: Bottle,
}

#[derive(Deserialize)]
struct Formulae(Vec<Formula>);

#[derive(Deserialize)]
struct BottleInfo {
    url: String,
    sha256: String,
}

fn d(f: &Formula) -> Option<()> {
    if f.versions.bottle {
        let name = &f.name;
        let ver = f.versions.stable.as_ref()?;
        let bs = f.bottle.stable.as_ref()?;
        let rebuild = bs.rebuild;
        for (platform, v) in bs.files.as_object()?.iter() {
            if let Ok(bi) = serde_json::from_value::<BottleInfo>(v.clone()) {
                if rebuild == 0 {
                    println!("{} {} {}-{}.{}.bottle.tar.gz", bi.sha256, bi.url, name, ver, platform);
                } else {
                    println!("{} {} {}-{}.{}.bottle.{}.tar.gz", bi.sha256, bi.url, name, ver, platform, rebuild);
                }
            }
        }
    }
    Some(())
}

fn main() {
    let f: Formulae = serde_json::from_reader(std::io::stdin()).unwrap();
    for f in f.0 {
        d(&f);
    }
}
