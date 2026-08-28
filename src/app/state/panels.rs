use super::{SettingsState, TransitionPreviewState};

#[derive(Default)]
pub(crate) struct PanelsState {
    pub(crate) audio_active: bool,
    pub(crate) audio_playing: bool,
    pub(crate) audio: Option<crate::frontend::audio_panel::AudioPanel>,
    pub(crate) settings: SettingsState,
    pub(crate) transition_preview: TransitionPreviewState,
    pub(crate) playlists: Option<crate::frontend::playlists::Playlists>,
    pub(crate) card_picker: Option<crate::frontend::playlists::CardPicker>,
    pub(crate) effects: Option<crate::frontend::effects::Effects>,
    pub(crate) schedule: Option<crate::frontend::schedule_editor::ScheduleEditor>,
    pub(crate) theme_designer: Option<crate::frontend::theme_designer::ThemeDesigner>,
    pub(crate) scene_properties: Option<crate::frontend::scene_properties::SceneProperties>,
}
