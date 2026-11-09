use whisker::css::{AlignItems, Border, BorderStyle, Color, FlexDirection};
use whisker::prelude::*;

pub const BG_CANVAS: u32 = 0x0c0e14;
pub const BG_CARD: u32 = 0x161922;
pub const BG_CARD_SURFACE: u32 = 0x1e2330;
pub const BG_INPUT: u32 = 0x11131b;
pub const BORDER_COLOR: u32 = 0x292f3f;
pub const TEXT_PRIMARY: u32 = 0xf8fafc;
pub const TEXT_SECONDARY: u32 = 0x94a3b8;
pub const TEXT_MUTED: u32 = 0x64748b;
pub const ACCENT_PRIMARY: u32 = 0x6366f1;
pub const ACCENT_INCOME: u32 = 0x10b981;
pub const ACCENT_EXPENSE: u32 = 0xf43f5e;
pub const ACCENT_WARNING: u32 = 0xf59e0b;

pub fn column() -> Css {
    Css::new()
        .display_flex()
        .flex_direction(FlexDirection::Column)
}

pub fn row() -> Css {
    Css::new()
        .display_flex()
        .flex_direction(FlexDirection::Row)
        .align_items(AlignItems::Center)
}

pub fn card_style() -> Css {
    column()
        .background_color(Color::hex(BG_CARD))
        .border_radius(px(16))
        .border(
            Border::new()
                .width(px(1))
                .style(BorderStyle::Solid)
                .color(Color::hex(BORDER_COLOR)),
        )
        .padding(px(18))
}

pub fn subtle_border() -> Border {
    Border::new()
        .width(px(1))
        .style(BorderStyle::Solid)
        .color(Color::hex(BORDER_COLOR))
}
