use crate::icons::Icons;
use crate::theme::CapBdTheme;
use capbd_core::history::HistoryManager;
use capbd_core::project::Project;
use capbd_core::timeline::AspectRatio;
use egui::{vec2, Align, Color32, Layout, RichText, Ui};

pub struct TopBar;

pub enum TopBarAction {
    None,
    Undo,
    Redo,
    TakeSnapshot,
    OpenExportModal,
}

impl TopBar {
    pub fn render(
        ui: &mut Ui,
        project: &mut Project,
        history: &HistoryManager,
        show_export_modal: &mut bool,
    ) -> TopBarAction {
        let mut action = TopBarAction::None;

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = vec2(12.0, 0.0);

            // 1. CapBD Brand Logo & Title
            let logo_size = vec2(28.0, 28.0);
            let (logo_rect, _) = ui.allocate_exact_size(logo_size, egui::Sense::hover());
            Icons::draw_capbd_logo(&ui.painter_at(logo_rect), logo_rect);

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("CapBD")
                            .color(Color32::WHITE)
                            .strong()
                            .size(15.0),
                    );
                    ui.label(
                        RichText::new("Editor")
                            .color(CapBdTheme::ACCENT_CYAN)
                            .size(12.0),
                    );
                });
            });

            ui.separator();

            // 2. Project Name Editable Input
            ui.add(
                egui::TextEdit::singleline(&mut project.metadata.name)
                    .desired_width(180.0)
                    .font(egui::TextStyle::Body)
                    .hint_text("Project Title..."),
            );

            ui.separator();

            // 3. Undo / Redo
            let undo_resp = Icons::button_custom(
                ui,
                "undo_btn",
                vec2(28.0, 28.0),
                "Undo (Ctrl+Z)",
                |painter, rect, color| {
                    let col = if history.can_undo() {
                        color
                    } else {
                        CapBdTheme::TEXT_MUTED
                    };
                    Icons::draw_undo(painter, rect, col);
                },
            );
            if undo_resp.clicked() && history.can_undo() {
                action = TopBarAction::Undo;
            }

            let redo_resp = Icons::button_custom(
                ui,
                "redo_btn",
                vec2(28.0, 28.0),
                "Redo (Ctrl+Y)",
                |painter, rect, color| {
                    let col = if history.can_redo() {
                        color
                    } else {
                        CapBdTheme::TEXT_MUTED
                    };
                    Icons::draw_redo(painter, rect, col);
                },
            );
            if redo_resp.clicked() && history.can_redo() {
                action = TopBarAction::Redo;
            }

            ui.separator();

            // 4. Aspect Ratio Selector (16:9, 9:16, 1:1, etc.)
            let current_aspect = project.timeline.aspect_ratio;
            egui::ComboBox::from_id_salt("aspect_ratio_combo")
                .selected_text(
                    RichText::new(current_aspect.short_name()).color(CapBdTheme::ACCENT_CYAN),
                )
                .show_ui(ui, |ui| {
                    let ratios = [
                        AspectRatio::Ratio16x9,
                        AspectRatio::Ratio9x16,
                        AspectRatio::Ratio1x1,
                        AspectRatio::Ratio4x5,
                        AspectRatio::Ratio21x9,
                    ];
                    for r in ratios {
                        if ui
                            .selectable_value(&mut project.timeline.aspect_ratio, r, r.label())
                            .clicked()
                        {
                            // Changed aspect ratio
                        }
                    }
                });

            // 5. Right-Aligned Action Buttons (Snapshot & Export)
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                // Export Button (Primary Cyan Pill)
                let export_btn = egui::Button::new(
                    RichText::new("Export")
                        .color(Color32::BLACK)
                        .strong()
                        .size(13.0),
                )
                .fill(CapBdTheme::ACCENT_CYAN)
                .rounding(egui::Rounding::same(14.0_f32));

                if ui.add(export_btn).clicked() {
                    *show_export_modal = true;
                    action = TopBarAction::OpenExportModal;
                }

                // Snapshot Button
                if ui
                    .button(
                        RichText::new("Snapshot")
                            .color(CapBdTheme::TEXT_PRIMARY)
                            .size(12.0),
                    )
                    .clicked()
                {
                    action = TopBarAction::TakeSnapshot;
                }
            });
        });

        action
    }
}
