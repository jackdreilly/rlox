use goldenfile::Mint;
use rlox::scanner::Scanned;
use std::fs;
use std::io::{Result, Write};

#[test]
fn golden_tests() -> Result<()> {
    let mut scan_mint = Mint::new("tests/goldenfiles/scans");
    for file in fs::read_dir("tests/programs")? {
        let path = file?.path();
        let file_name = path.file_name().unwrap();
        let mut minted = scan_mint.new_goldenfile(file_name)?;
        let program = fs::read_to_string(path)?;
        let scanned = program.as_str().scan();
        for token in scanned {
            writeln!(minted, "{:#?}", token)?;
        }
    }
    Ok(())
}
