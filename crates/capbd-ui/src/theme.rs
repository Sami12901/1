use egui::{Color32, CornerRadius, Margin, Stroke, Visuals};

pub struct CapBdTheme;

impl CapBdTheme {
    // CapCut Signature Dark Palette
    pub const BG_APP: Color32 = Color32::from_rgb(15, 15, 18);
    pub const BG_PANEL: Color32 = Color32::from_rgb(24, 24, 30);
    pub const BG_HEADER: Color32 = Color32::from_rgb(20, 20, 25);
    pub const BG_CARD: Color32 = Color32::from_rgb(32, 32, 40);
    pub const BG_INPUT: Color32 = Color32::from_rgb(18, 18, 22);

    pub const BORDER: Color32 = Color32::from_rgb(44, 44, 54);
    pub const BORDER_FOCUS: Color32 = Color32::from_rgb(0, 229, 255);

    // Accents
    pub const ACCENT_CYAN: Color32 = Color32::from_rgb(0, 242, 254);
    pub const ACCENT_BLUE: Color32 = Color32::from_rgb(0, 114, 255);
    pub const ACCENT_RED: Color32 = Color32::from_rgb(255, 69, 58);
    pub const ACCENT_GREEN: Color32 = Color32::from_rgb(48, 209, 88);
    pub const ACCENT_YELLOW: Color32 = Color32::from_rgb(255, 214, 10);

    // Track & Clip Colors
    pub const CLIP_VIDEO: Color32 = Color32::from_rgb(0, 150, 220);
    pub const CLIP_AUDIO: Color32 = Color32::from_rgb(30, 190, 110);
    pub const CLIP_TEXT: Color32 = Color32::from_rgb(170, 70, 240);
    pub const CLIP_EFFECT: Color32 = Color32::from_rgb(255, 140, 20);

    // Text
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(255, 255, 255);
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(158, 158, 167);
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(100, 100, 112);

    pub fn apply(ctx: &egui::Context) {
        let mut visuals = Visuals::dark();

        visuals.override_text_color = Some(Self::TEXT_PRIMARY);
        visuals.panel_fill = Self::BG_PANEL;
        visuals.window_fill = Self::BG_APP;

        // Widgets
        visuals.widgets.noninteractive.bg_fill = Self::BG_PANEL;
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, Self::BORDER);
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Self::TEXT_PRIMARY);
        visuals.widgets.noninteractive.corner_radius = CornerRadius::same(6);

        visuals.widgets.inactive.bg_fill = Self::BG_CARD;
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Self::BORDER);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Self::TEXT_PRIMARY);
        visuals.widgets.inactive.corner_radius = CornerRadius::same(6);

        visuals.widgets.hovered.bg_fill = Color32::from_rgb(42, 42, 54);
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Self::ACCENT_CYAN);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::WHITE);
        visuals.widgets.hovered.corner_radius = CornerRadius::same(6);

        visuals.widgets.active.bg_fill = Self::ACCENT_BLUE;
        visuals.widgets.active.bg_stroke = Stroke::new(1.0, Self::ACCENT_CYAN);
        visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
        visuals.widgets.active.corner_radius = CornerRadius::same(6);

        visuals.selection.bg_fill = Self::ACCENT_BLUE;
        visuals.selection.stroke = Stroke::new(1.5, Self::ACCENT_CYAN);

        ctx.set_visuals(visuals);

        // Adjust style spacing
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        style.spacing.window_margin = Margin::same(12);
        style.spacing.button_padding = egui::vec2(10.0, 6.0);
        ctx.set_style(style);
    }
}
