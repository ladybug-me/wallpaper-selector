use crate::contracts::picker::KEY_BINDINGS;
use crate::domain::input::InputMap;

use super::Config;

pub fn load_bindings(config: &Config) -> InputMap {
    InputMap::from_bindings(
        KEY_BINDINGS
            .into_iter()
            .map(|descriptor| (descriptor.action, config.str_path(descriptor.path))),
    )
}

mod tests;
