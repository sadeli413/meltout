use super::console::parsers;
use super::errors::Error;

pub trait Commander {
    fn generate(&self, parser: parsers::Generate) -> Result<(), Error>;
    fn listeners(&mut self, parser: parsers::Listeners) -> Result<(), Error>;
    fn task(&mut self, parser: parsers::Task) -> Result<(), Error>;
    fn implants(&mut self, parser: parsers::Implants) -> Result<(), Error>;
}

pub type Handler<T> = fn(&mut T, Vec<&str>) -> Result<(), Error>;

// Macro to add the a command to the console
#[macro_export]
macro_rules! add_command {
    ($console:ident, $parsers:ident, $cmd:ident) => {{
        paste! {
            $console
                .commands
                .insert(stringify!($cmd).to_string(), |meltout, line| {
                    let parser = $parsers::[<$cmd:camel>]::try_parse_from(line)
                        .map_err(|e| Error::CommandParsingErr(e))?;
                    meltout.$cmd(parser)?;
                    Ok(())
                });
        }
    }};
}
