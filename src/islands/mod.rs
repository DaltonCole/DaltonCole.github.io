//! The interactive tools. This is the only code compiled to WebAssembly.

mod crt;
mod murder_mystery;
mod phase10;

pub use crt::ChineseRemainderSolver;
pub use murder_mystery::MurderMysteryGame;
pub use phase10::PhaseTenScoreKeeper;
