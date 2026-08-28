use serde_json::Value;

use crate::contracts::daemon::{EffectOperationResult, EffectsListResult};
use crate::domain::effects::EffectDefinition;

use super::common::{DecodeResult, array, envelope, required_string};

pub fn decode_effect_operation(value: &Value) -> DecodeResult<EffectOperationResult> {
    let object = envelope("effects.operation", value)?;
    Ok(EffectOperationResult { output: required_string("effects.operation", object, "output")? })
}

pub fn decode_effects_list(value: &Value) -> DecodeResult<EffectsListResult> {
    let object = envelope("effects.list", value)?;
    let definitions = array("effects.list", object, "effects")?
        .map(crate::infrastructure::effects::decode_definitions);
    let theme_options = definitions.as_deref().and_then(theme_options);
    Ok(EffectsListResult { definitions, theme_options })
}

fn theme_options(definitions: &[EffectDefinition]) -> Option<Vec<String>> {
    let mut result = None;
    for definition in definitions.iter().filter(|definition| definition.id == "theme") {
        for parameter in definition.params.iter().filter(|parameter| parameter.id == "theme") {
            result = Some(parameter.options.iter().map(|option| option.mode.clone()).collect());
        }
    }
    result
}
