use super::models::CustomEditorColors;
use crate::ui::layout::theme::MorTheme;

pub fn resolve_effective_theme(base: MorTheme, overrides: &CustomEditorColors) -> MorTheme {
    macro_rules! apply {
        ($field:ident) => {
            overrides.$field.clone().unwrap_or(base.$field)
        };
    }
    MorTheme {
        bg: apply!(bg),
        panel: apply!(panel),
        header: apply!(header),
        text: apply!(text),
        text_muted: apply!(text_muted),
        border: apply!(border),
        border_light: apply!(border_light),
        accent: apply!(accent),
        accent_hover: apply!(accent_hover),
        btn: apply!(btn),
        btn_hover: apply!(btn_hover),
        font_family: apply!(font_family),
        font_size_base: apply!(font_size_base),
        font_size_h1: apply!(font_size_h1),
        padding_base: apply!(padding_base),
        border_radius: apply!(border_radius),
        destructive: apply!(destructive),
        success: apply!(success),
        warning: apply!(warning),
        // All non-overridable fields pass through from base unchanged.
        ..base
    }
}
