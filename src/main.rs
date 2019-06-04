fn main() -> Result<(), failure::Error> {
    env_logger::init();

    log::info!("pwd {}", std::env::current_dir()?.display());
    shell::cmd!("vim").run()?;

    Ok(())
}
