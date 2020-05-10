use std::fs::File;
use std::error::Error;
use std::str;
use pancakestack::PancakeStack;

#[test]
fn simplest() -> Result<(), Box<dyn Error>> {
    let program = "Put this test pancake on top!\nShow me a pancake!";
    let program_buf = program.as_bytes();
    
    let mut output_buf = Vec::new();
    PancakeStack::new().run_program(program_buf, std::io::empty(), &mut output_buf);
    let output = str::from_utf8(&output_buf)?;
    assert_eq!(output, "\x04");
    Ok(())
}

#[test]
fn hello_world() -> Result<(), Box<dyn Error>> {
    let file = File::open("tests/hello_world.pancake")?;
    let mut output_buf = Vec::new();
    PancakeStack::new().run_program(file, std::io::empty(), &mut output_buf);
    let output = str::from_utf8(&output_buf)?;
    assert_eq!(output, "Hello World!");
    Ok(())
}

#[test]
fn cat() -> Result<(), Box<dyn Error>> {
    let file = File::open("tests/cat.pancake")?;
    let input = "hFeuofpegiurbfoieboiejfpeiDIWUHFUIwfjoweopUBUfbeufbiubfiuwebfowmFMPIEFBO UF EFEW\x00";
    let input_buf = input.as_bytes();
    let mut output_buf = Vec::new();
    PancakeStack::new().run_program(file, input_buf, &mut output_buf);
    let output = str::from_utf8(&output_buf)?;
    assert_eq!(output, input);
    Ok(())
}


