// services/mod.rs
pub mod overlay;

pub trait Service {
    fn start(&self);

    fn stop(&self);
}