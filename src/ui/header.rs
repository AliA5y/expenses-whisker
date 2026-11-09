use whisker::css::{AlignItems, Color, Cursor, FontWeight, JustifyContent};
use whisker::prelude::*;
use whisker_icons::{Icon, lucide};

use crate::store::AppState;
use crate::theme::{self, ACCENT_PRIMARY, BG_CARD, TEXT_MUTED, TEXT_PRIMARY};

#[component]
pub fn app_header(state: AppState) -> Element {
    let add_tap = move |_| {
        state.is_add_modal_open.set(true);
    };
    let reset_tap = move |_| {
        state.reset_defaults();
    };

    render! {
        View(
            style: theme::row()
                .justify_content(JustifyContent::SpaceBetween)
                .align_items(AlignItems::Center)
                .padding_bottom(px(14))
                .border_bottom(theme::subtle_border()),
        ) {
            View(style: theme::row().gap(px(10)).align_items(AlignItems::Center)) {
                View(
                    style: theme::row()
                        .justify_content(JustifyContent::Center)
                        .align_items(AlignItems::Center)
                        .width(px(38))
                        .height(px(38))
                        .border_radius(px(10))
                        .background_color(Color::hex(ACCENT_PRIMARY)),
                ) {
                    Icon(svg: lucide::Wallet, color: "#ffffff", size: "20")
                }
                View(style: theme::column().gap(px(1))) {
                    Text(
                        value: "ExpenseFlow",
                        style: Css::new()
                            .color(Color::hex(TEXT_PRIMARY))
                            .font_size(px(19))
                            .font_weight(FontWeight::Numeric(700)),
                    )
                    Text(
                        value: "Finance & Budget",
                        style: Css::new().color(Color::hex(TEXT_MUTED)).font_size(px(11)),
                    )
                }
            }

            View(style: theme::row().gap(px(8))) {
                View(
                    style: theme::row()
                        .gap(px(4))
                        .padding_left(px(10))
                        .padding_right(px(10))
                        .padding_top(px(6))
                        .padding_bottom(px(6))
                        .border_radius(px(8))
                        .background_color(Color::hex(BG_CARD))
                        .border(theme::subtle_border())
                        .cursor(Cursor::Pointer),
                    on_tap: reset_tap,
                ) {
                    Icon(svg: lucide::RotateCcw, color: "#94a3b8", size: "13")
                    Text(
                        value: "Reset",
                        style: Css::new().color(Color::hex(TEXT_MUTED)).font_size(px(12)),
                    )
                }

                View(
                    style: theme::row()
                        .gap(px(6))
                        .padding_left(px(12))
                        .padding_right(px(12))
                        .padding_top(px(8))
                        .padding_bottom(px(8))
                        .border_radius(px(10))
                        .background_color(Color::hex(ACCENT_PRIMARY))
                        .cursor(Cursor::Pointer),
                    on_tap: add_tap,
                ) {
                    Icon(svg: lucide::Plus, color: "#ffffff", size: "15")
                    Text(
                        value: "Add",
                        style: Css::new()
                            .color(Color::hex(0xffffff))
                            .font_size(px(13))
                            .font_weight(FontWeight::Numeric(600)),
                    )
                }
            }
        }
    }
}
