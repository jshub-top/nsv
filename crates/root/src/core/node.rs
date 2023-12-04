use super::NsvCore;

pub enum UseVersionError {}

pub trait UseVersion {
    fn use_version(&self, version: &'static str) -> Result<(), UseVersionError>;
}

impl UseVersion for NsvCore {
    fn use_version(&self, _version: &'static str) -> Result<(), UseVersionError> {


        // self.context.
        Ok(())
    }
}
