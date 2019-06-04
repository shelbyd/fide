fn main() -> Result<(), failure::Error> {
    shell::cmd!("vim").run()?;
    Ok(())
}
