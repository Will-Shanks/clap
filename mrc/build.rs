use clap::CommandFactory;
use clap_mangen::Man;
use std::{
    env,
    fs::File,
    io::Error,
    path::Path,
};

include!("src/cli.rs");

fn build_page(outdir: &Path, app: clap::Command) {
    let name = app.get_display_name().unwrap_or_else(|| app.get_name());
    let file = Path::new(&outdir).join(format!("{}.1", name));
    let mut file = File::create(file).unwrap();

    Man::new(app.clone()).render(&mut file).unwrap();

    for sub in app.get_subcommands() {
        build_page(outdir, sub.clone());
    }
}

fn build_manpages(outdir: &Path) -> Result<(), Error> {
    let mut app = Opt::command();
    app.build();
    let outdir = outdir.join("man");
    std::fs::create_dir_all(&outdir).unwrap();
    build_page(&outdir, app);


    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=man");

    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    // Create `target/assets/` folder.
    let out_path = PathBuf::from(outdir);
    let mut path = out_path.ancestors().nth(4).unwrap().to_owned();
    path.push("assets");
    std::fs::create_dir_all(&path).unwrap();

    build_manpages(&path)?;

    Ok(())
}

