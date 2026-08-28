#[derive(Clone)]
pub enum BrowserAct {
    Close,
    Category(u8),
    Sort(&'static str),
    Purity(u8),
    Color(i64),
    TopRange(&'static str),
    ResMode,
    Atleast(&'static str),
    Atmost(&'static str),
    Label,
    Ratios(&'static str),
    Collection(String),
    MaxDuration(&'static str),
    SteamFilter(&'static str, String),
    CatalogFilter(&'static str, String),
}
