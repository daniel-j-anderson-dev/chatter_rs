use std::io::{stdin, stdout, Write};

pub fn read_line(prompt: &str) -> Result<String, std::io::Error> {
    let mut stdout = stdout();
    stdout.write(prompt.as_bytes())?;
    stdout.flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let input = input.trim().to_owned();

    Ok(input)
}
