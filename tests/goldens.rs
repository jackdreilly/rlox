use goldenfile::Mint;
use rlox::chunk_printer::print_chunk;
use rlox::compiler::Compiled;
use rlox::scanner::Scanned;
use std::io::{ErrorKind, Result, Write};
use std::{fs, io};

#[test]
fn golden_tests() -> Result<()> {
    let mut scan_mint = Mint::new("tests/goldenfiles/scans");
    let mut compile_mint = Mint::new("tests/goldenfiles/chunks");
    for file in fs::read_dir("tests/programs")? {
        let path = file?.path();
        let path_clone = path.clone();
        let file_name_string = path.file_name().unwrap().to_str().unwrap();
        let mut minted = scan_mint.new_goldenfile(file_name_string)?;
        let program = fs::read_to_string(path_clone)?;
        let scanned = program.as_str().scan();
        for token in scanned {
            writeln!(minted, "{:#?}", token)?;
        }
        if !vec!["num.lox".to_string()].contains(&file_name_string.to_string()) {
            continue;
        }
        let mut compile_minted = compile_mint.new_goldenfile(file_name_string)?;

        let scanned = program
            .as_str()
            .compile()
            .map_err(|e| io::Error::new(ErrorKind::Other, e.message))?;
        print_chunk(&scanned, &mut compile_minted, file_name_string)?;
    }
    Ok(())
}
