#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PaletteSpec {
    pub primary: Option<String>,
    pub primary_text: Option<String>,
    pub surface: Option<String>,
    pub surface_text: Option<String>,
    pub surface_variant: Option<String>,
    pub surface_container: Option<String>,
    pub background: Option<String>,
    pub outline: Option<String>,
    pub tertiary: Option<String>,
}

#[cfg(test)]
mod tests;
