// #[cfg(feature = "param-patch")]
// pub mod parse;
mod ast; pub use ast::*;


pub fn calculate_db(db: f64) -> f32 { 10.0_f64.powf(db * (1.0 / 20.0)) as f32 }