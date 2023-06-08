use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::str;

use pancakestack::Command;

#[test]
fn simplest() -> Result<(), Box<dyn Error>> {
    let program = "Put this test pancake on top!\nShow me a pancake!";

    let mut output_buf = Vec::new();
    pancakestack::run_program_str(program, std::io::empty(), &mut output_buf)?;
    let output = str::from_utf8(&output_buf)?;
    assert_eq!(output, "\x04");
    Ok(())
}

#[test]
fn parse_program() -> Result<(), Box<dyn Error>> {
    let program_str = "Put this test pancake on top!\nShow me a pancake!";
    let program_parsed = pancakestack::parse_program_str(program_str);

    assert_eq!(
        program_parsed,
        vec![
            Command::PutThisPancakeOnTop("test".into()),
            Command::ShowMeAPancake
        ]
    );
    Ok(())
}

#[test]
fn hello_world() -> Result<(), Box<dyn Error>> {
    let file = File::open("examples/hello_world.pancake")?;
    let mut output_buf = Vec::new();
    pancakestack::run_program_from_read(file, std::io::empty(), &mut output_buf)?;
    let output = str::from_utf8(&output_buf)?;
    assert_eq!(output, "Hello World!");
    Ok(())
}

#[test]
fn countdown_bin() -> Result<(), Box<dyn Error>> {
    let file = File::open("examples/countdown_bin.pancake")?;
    let mut input = 100;
    let mut output = Vec::new();
    pancakestack::run_program_from_read(file, &[input][..], &mut output)?;

    let mut expected = Vec::with_capacity(input as usize);
    loop {
        expected.push(input);
        if input == 0 {
            break;
        }
        input -= 1;
    }

    assert_eq!(output, expected);
    Ok(())
}

#[test]
fn div10() -> Result<(), Box<dyn Error>> {
    let file = File::open("examples/div10.pancake")?;
    let input = b"87";
    let mut output = Vec::new();
    pancakestack::run_program_from_read(file, &input[..], &mut output)?;
    println!("{:#?}", &output);
    assert_eq!(output, b"\x08");
    Ok(())
}

#[test]
fn cat_program_parsed() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("examples/cat.pancake")?;
    let mut program_str = String::new();
    file.read_to_string(&mut program_str)?;
    let program = pancakestack::parse_program_str(&program_str);
    let input =
        b"hFeuofpegiurbfoieboiejfpeiDIWUHFUIwfjoweopUBUfbeufbiubfiuwebfowmFMPIEFBO UF EFEW\x00";
    let mut output = Vec::new();
    pancakestack::run_program(&program, &input[..], &mut output)?;
    assert_eq!(output, &input[..]);
    Ok(())
}

#[test]
fn cat_program_str() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("examples/cat.pancake")?;
    let mut program = String::new();
    file.read_to_string(&mut program)?;

    let input =
        b"hFeuofpegiurbfoieboiejfpeiDIWUHFUIwfjoweopUBUfbeufbiubfiuwebfowmFMPIEFBO UF EFEW\x00";

    let mut output = Vec::new();
    pancakestack::run_program_str(&program, &input[..], &mut output)?;
    assert_eq!(output, &input[..]);
    Ok(())
}

#[test]
fn cat_program_read() -> Result<(), Box<dyn Error>> {
    let file = File::open("examples/cat.pancake")?;
    let input =
        b"hFeuofpegiurbfoieboiejfpeiDIWUHFUIwfjoweopUBUfbeufbiubfiuwebfowmFMPIEFBO UF EFEW\x00";
    let mut output = Vec::new();
    pancakestack::run_program_from_read(file, &input[..], &mut output)?;
    assert_eq!(output, &input[..]);
    Ok(())
}
