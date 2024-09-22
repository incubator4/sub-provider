pub mod clash;

pub trait Provider {
    fn provide(&self) -> String;
}
