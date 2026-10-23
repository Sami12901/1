use egui::{epaint::PathShape, Color32, Painter, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};

pub struct Icons;

impl Icons {
    /// Renders a clickable icon button with custom vector geometry
    pub fn button_custom(
        ui: &mut Ui,
        _id_source: impl std::hash::Hash,
        size: Vec2,
        tooltip: &str,
        render_icon: impl FnOnce(&Painter, Rect, Color32),
    ) -> Response {
        let (rect, response) = ui.allocate_exact_size(size, Sense::click());
        let painter = ui.painter_at(rect);

        let bg_color = if response.is_pointer_button_down_on() {
            Color32::from_rgb(40, 50, 70)
        } else if response.hovered() {
            Color32::from_rgb(34, 38, 48)
        } else {
            Color32::TRANSPARENT
        };

        if bg_color != Color32::TRANSPARENT {
            painter.rect_filled(rect, 4.0, bg_color);
        }

        let icon_color = if response.hovered() {
            Color32::from_rgb(0, 242, 254)
        } else {
            Color32::from_rgb(220, 224, 230)
        };

        render_icon(&painter, rect, icon_color);

        if !tooltip.is_empty() {
            response.on_hover_text(tooltip)
        } else {
            response
        }
    }

    /// Renders the signature CapBD Geometric Logo Mark
    pub fn draw_capbd_logo(painter: &Painter, rect: Rect) {
        let center = rect.center();
        let s = rect.width().min(rect.height()) * 0.45;

        // Background squircle
        painter.rect_filled(rect, 8.0, Color32::from_rgb(14, 14, 18));
        painter.rect_stroke(
            rect,
            8.0,
            Stroke::new(1.0_f32, Color32::from_rgb(38, 42, 54)),
        );

        // Upper Blade (White)
        let top_points = vec![
            Pos2::new(center.x - s * 0.7, center.y - s * 0.7),
            Pos2::new(center.x + s * 0.5, center.y - s * 0.7),
            Pos2::new(center.x + s * 0.2, center.y - s * 0.1),
            Pos2::new(center.x - s * 0.4, center.y - s * 0.1),
        ];
        painter.add(PathShape::convex_polygon(
            top_points,
            Color32::WHITE,
            Stroke::NONE,
        ));

        // Lower Blade (Electric Cyan)
        let bottom_points = vec![
            Pos2::new(center.x + s * 0.7, center.y + s * 0.7),
            Pos2::new(center.x - s * 0.5, center.y + s * 0.7),
            Pos2::new(center.x - s * 0.2, center.y + s * 0.1),
            Pos2::new(center.x + s * 0.4, center.y + s * 0.1),
        ];
        painter.add(PathShape::convex_polygon(
            bottom_points,
            Color32::from_rgb(0, 242, 254),
            Stroke::NONE,
        ));

        // Center Diagonal Slash Cut
        painter.line_segment(
            [
                Pos2::new(center.x - s * 0.6, center.y - s * 0.05),
                Pos2::new(center.x + s * 0.6, center.y - s * 0.05),
            ],
            Stroke::new(2.0_f32, Color32::from_rgb(14, 14, 18)),
        );
    }

    /// Play Icon (Solid Right Arrow)
    pub fn draw_play(painter: &Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        let s = rect.width().min(rect.height()) * 0.28;
        let points = vec![
            Pos2::new(c.x - s * 0.6, c.y - s),
            Pos2::new(c.x + s * 0.8, c.y),
            Pos2::new(c.x - s * 0.6, c.y + s),
        ];
        painter.add(PathShape::convex_polygon(points, color, Stroke::NONE));
    }

    /// Pause Icon (Dual Vertical Bars)
    pub fn draw_pause(painter: &Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        let s = rect.width().min(rect.height()) * 0.28;
        let bar_w = s * 0.5;
        let gap = s * 0.4;

        let left_rect = Rect::from_min_max(
            Pos2::new(c.x - gap - bar_w, c.y - s),
            Pos2::new(c.x - gap, c.y + s),
        );
        let right_rect = Rect::from_min_max(
            Pos2::new(c.x + gap, c.y - s),
            Pos2::new(c.x + gap + bar_w, c.y + s),
        );

        painter.rect_filled(left_rect, 1.5, color);
        painter.rect_filled(right_rect, 1.5, color);
    }

    /// Step Backward 1 Frame
    pub fn draw_step_back(painter: &Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        let s = rect.width().min(rect.height()) * 0.26;

        // Bar on left
        painter.rect_filled(
            Rect::from_min_max(
                Pos2::new(c.x - s - 2.0, c.y - s),
                Pos2::new(c.x - s, c.y + s),
            ),
            1.0,
            color,
        );

        // Arrow pointing left
        let points = vec![
            Pos2::new(c.x + s * 0.6, c.y - s),
            Pos2::new(c.x - s * 0.6, c.y),
            Pos2::new(c.x + s * 0.6, c.y + s),
        ];
        painter.add(PathShape::convex_polygon(points, color, Stroke::NONE));
    }

    /// Step Forward 1 Frame
    pub fn draw_step_forward(painter: &Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        let s = rect.width().min(rect.height()) * 0.26;

        // Bar on right
        painter.rect_filled(
            Rect::from_min_max(
                Pos2::new(c.x + s, c.y - s),
                Pos2::new(c.x + s + 2.0, c.y + s),
            ),
            1.0,
            color,
        );

        // Arrow pointing right
        let points = vec![
            Pos2::new(c.x - s * 0.6, c.y - s),
            Pos2::new(c.x + s * 0.6, c.y),
            Pos2::new(c.x - s * 0.6, c.y + s),
        ];
        painter.add(PathShape::convex_polygon(points, color, Stroke::NONE));
    }

    /// Scissors Split Tool
    pub fn draw_scissors(painter: &Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        let s = rect.width().min(rect.height()) * 0.3;

        // Two intersecting blade lines
        painter.line_segment(
            [
                Pos2::new(c.x - s, c.y - s * 0.8),
                Pos2::new(c.x + s, c.y + s * 0.8),
            ],
            Stroke::new(2.0_f32, color),
        );
        painter.line_segment(
            [
                Pos2::new(c.x - s, c.y + s * 0.8),
                Pos2::new(c.x + s, c.y - s * 0.8),
            ],
            Stroke::new(2.0_f32, color),
        );

        // Pivot screw
        painter.circle_filled(c, 2.5, Color32::from_rgb(0, 242, 254));
    }

    /// Trash Can / Delete Icon
    pub fn draw_trash(painter: &Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        let s = rect.width().min(rect.height()) * 0.28;

        // Lid
        painter.line_segment(
            [
                Pos2::new(c.x - s, c.y - s * 0.7),
                Pos2::new(c.x + s, c.y - s * 0.7),
            ],
            Stroke::new(2.0_f32, color),
        );
        // Can body
        painter.rect_stroke(
            Rect::from_min_max(
                Pos2::new(c.x - s * 0.7, c.y - s * 0.5),
                Pos2::new(c.x + s * 0.7, c.y + s),
            ),
            2.0,
            Stroke::new(1.8_f32, color),
        );
    }

    /// Undo Icon (Curved Arrow Counter-Clockwise)
    pub fn draw_undo(painter: &Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        let s = rect.width().min(rect.height()) * 0.28;

        // Arrow head pointing left
        let head = vec![
            Pos2::new(c.x - s * 0.8, c.y - s * 0.2),
            Pos2::new(c.x - s * 0.2, c.y - s * 0.8),
            Pos2::new(c.x - s * 0.2, c.y + s * 0.4),
        ];
        painter.add(PathShape::convex_polygon(head, color, Stroke::NONE));

        // Curved tail arc
        painter.line_segment(
            [
                Pos2::new(c.x - s * 0.2, c.y - s * 0.2),
                Pos2::new(c.x + s * 0.6, c.y - s * 0.2),
            ],
            Stroke::new(2.0_f32, color),
        );
        painter.line_segment(
            [
                Pos2::new(c.x + s * 0.6, c.y - s * 0.2),
                Pos2::new(c.x + s * 0.6, c.y + s * 0.6),
            ],
            Stroke::new(2.0_f32, color),
        );
    }

    /// Redo Icon (Curved Arrow Clockwise)
    pub fn draw_redo(painter: &Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        let s = rect.width().min(rect.height()) * 0.28;

        // Arrow head pointing right
        let head = vec![
            Pos2::new(c.x + s * 0.8, c.y - s * 0.2),
            Pos2::new(c.x + s * 0.2, c.y - s * 0.8),
            Pos2::new(c.x + s * 0.2, c.y + s * 0.4),
        ];
        painter.add(PathShape::convex_polygon(head, color, Stroke::NONE));

        // Curved tail arc
        painter.line_segment(
            [
                Pos2::new(c.x + s * 0.2, c.y - s * 0.2),
                Pos2::new(c.x - s * 0.6, c.y - s * 0.2),
            ],
            Stroke::new(2.0_f32, color),
        );
        painter.line_segment(
            [
                Pos2::new(c.x - s * 0.6, c.y - s * 0.2),
                Pos2::new(c.x - s * 0.6, c.y + s * 0.6),
            ],
            Stroke::new(2.0_f32, color),
        );
    }

    /// Magnet Snap Toggle Icon
    pub fn draw_magnet(painter: &Painter, rect: Rect, active: bool) {
        let c = rect.center();
        let s = rect.width().min(rect.height()) * 0.3;
        let color = if active {
            Color32::from_rgb(0, 242, 254)
        } else {
            Color32::from_rgb(120, 120, 130)
        };

        // Horseshoe U-shape
        painter.rect_stroke(
            Rect::from_min_max(Pos2::new(c.x - s, c.y - s), Pos2::new(c.x + s, c.y + s)),
            s * 0.6,
            Stroke::new(2.5_f32, color),
        );
        painter.rect_filled(
            Rect::from_min_max(
                Pos2::new(c.x - s * 0.4, c.y - s * 1.1),
                Pos2::new(c.x + s * 0.4, c.y + s * 0.4),
            ),
            0.0,
            Color32::from_rgb(24, 24, 30),
        );
    }

    /// Volume / Mute Icon
    pub fn draw_volume(painter: &Painter, rect: Rect, muted: bool) {
        let c = rect.center();
        let s = rect.width().min(rect.height()) * 0.28;
        let color = if muted {
            Color32::from_rgb(255, 69, 58)
        } else {
            Color32::from_rgb(220, 224, 230)
        };

        // Speaker box
        painter.rect_filled(
            Rect::from_min_max(
                Pos2::new(c.x - s, c.y - s * 0.4),
                Pos2::new(c.x - s * 0.4, c.y + s * 0.4),
            ),
            1.0,
            color,
        );
        // Speaker cone
        let cone = vec![
            Pos2::new(c.x - s * 0.4, c.y - s * 0.4),
            Pos2::new(c.x + s * 0.3, c.y - s * 0.9),
            Pos2::new(c.x + s * 0.3, c.y + s * 0.9),
            Pos2::new(c.x - s * 0.4, c.y + s * 0.4),
        ];
        painter.add(PathShape::convex_polygon(cone, color, Stroke::NONE));

        if muted {
            // Diagonal red slash across
            painter.line_segment(
                [
                    Pos2::new(c.x - s * 0.8, c.y + s * 0.8),
                    Pos2::new(c.x + s * 0.8, c.y - s * 0.8),
                ],
                Stroke::new(2.0_f32, Color32::from_rgb(255, 69, 58)),
            );
        } else {
            // Sound wave arc
            let p1 = Pos2::new(c.x + s * 0.6, c.y - s * 0.5);
            let p2 = Pos2::new(c.x + s * 0.8, c.y);
            let p3 = Pos2::new(c.x + s * 0.6, c.y + s * 0.5);
            painter.line_segment([p1, p2], Stroke::new(1.8_f32, color));
            painter.line_segment([p2, p3], Stroke::new(1.8_f32, color));
        }
    }
}
