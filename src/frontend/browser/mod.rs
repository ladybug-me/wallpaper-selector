mod model;
#[cfg(test)]
mod tests;
mod view;

pub use model::*;
pub(crate) use view::{BrowserCardAction, browser_card_action_at};
pub use view::{view, wall_aperture_dims};
