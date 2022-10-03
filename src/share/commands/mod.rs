use super::console::parsers;
use super::errors::Error;

pub trait Commander {
    fn generate(&self, parser: parsers::Generate) -> Result<(), Error>;
    fn listeners(&mut self, parser: parsers::Listeners) -> Result<(), Error>;
    fn task(&mut self, parser: parsers::Task) -> Result<(), Error>;
}

pub type Handler<T> = fn(&mut T, Vec<&str>) -> Result<(), Error>;
