use whisker::css::{AlignItems, Color, Cursor, FontWeight, JustifyContent, Overflow};
use whisker::prelude::*;
use whisker_icons::{Icon, lucide};

use crate::models::Category;
use crate::store::AppState;
use crate::theme::{self, BG_CARD_SURFACE, TEXT_MUTED, TEXT_PRIMARY};

#[component]
pub fn category_analytics(state: AppState) -> Element {
    let breakdown = computed(move || state.category_spending());
    let has_spending = computed(move || !breakdown.get().is_empty());

    render! {
        View(style: theme::card_style().gap(px(16))) {
            View(
                style: theme::row()
                    .justify_content(JustifyContent::SpaceBetween)
                    .align_items(AlignItems::Center),
            ) {
                View(style: theme::row().gap(px(8))) {
                    Icon(svg: lucide::ChartPie, color: "#94a3b8", size: "18")
                    Text(
                        value: "Spending Breakdown",
                        style: Css::new()
                            .color(Color::hex(TEXT_PRIMARY))
                            .font_size(px(16))
                            .font_weight(FontWeight::Numeric(600)),
                    )
                }
                Text(
                    value: "By Category",
                    style: Css::new().color(Color::hex(TEXT_MUTED)).font_size(px(12)),
                )
            }

            Show(
                when: move || has_spending.get(),
                fallback: move || render! {
                    View(
                        style: theme::row()
                            .justify_content(JustifyContent::Center)
                            .padding_top(px(20))
                            .padding_bottom(px(20)),
                    ) {
                        Text(
                            value: "No expense data yet for category breakdown",
                            style: Css::new().color(Color::hex(TEXT_MUTED)).font_size(px(13)),
                        )
                    }
                },
            ) {
                View(style: theme::column().gap(px(12))) {
                    ForEach(
                        each: move || breakdown.get(),
                        key: |item: &(Category, f64, f64)| item.0,
                        children: move |item: ReadSignal<(Category, f64, f64)>| {
                            let cat = computed(move || item.get().0);
                            let amt = computed(move || item.get().1);
                            let pct_val = computed(move || item.get().2);
                            let cat_tap = move |_| {
                                let c = cat.get();
                                if state.selected_category.get() == Some(c) {
                                    state.selected_category.set(None);
                                } else {
                                    state.selected_category.set(Some(c));
                                }
                            };
                        
                            render! {
                                View(
                                    style: theme::column().gap(px(6)).cursor(Cursor::Pointer),
                                    on_tap: cat_tap,
                                ) {
                                    View(
                                        style: theme::row()
                                            .justify_content(JustifyContent::SpaceBetween)
                                            .align_items(AlignItems::Center),
                                    ) {
                                        View(style: theme::row().gap(px(8))) {
                                            View(
                                                style: computed(move || {
                                                    theme::row()
                                                        .justify_content(JustifyContent::Center)
                                                        .align_items(AlignItems::Center)
                                                        .width(px(26))
                                                        .height(px(26))
                                                        .border_radius(px(6))
                                                        .background_color(Color::hex(cat.get().bg_color_hex()))
                                                }),
                                            ) {
                                                Icon(
                                                    svg: cat.get().icon_svg(),
                                                    color: "#ffffff",
                                                    size: "14",
                                                )
                                            }
                                            Text(
                                                value: computed(move || cat.get().display_name().to_string()),
                                                style: Css::new()
                                                    .color(Color::hex(TEXT_PRIMARY))
                                                    .font_size(px(13))
                                                    .font_weight(FontWeight::Numeric(500)),
                                            )
                                        }
                        
                                        View(style: theme::row().gap(px(8))) {
                                            Text(
                                                value: computed(move || format!("${:.2}", amt.get())),
                                                style: Css::new()
                                                    .color(Color::hex(TEXT_PRIMARY))
                                                    .font_size(px(13))
                                                    .font_weight(FontWeight::Numeric(600)),
                                            )
                                            Text(
                                                value: computed(move || format!("({:.1}%)", pct_val.get())),
                                                style: Css::new().color(Color::hex(TEXT_MUTED)).font_size(px(12)),
                                            )
                                        }
                                    }
                        
                                    // Category Bar
                                    View(
                                        style: Css::new()
                                            .display_flex()
                                            .height(px(6))
                                            .border_radius(px(3))
                                            .background_color(Color::hex(BG_CARD_SURFACE))
                                            .overflow(Overflow::Hidden),
                                    ) {
                                        View(
                                            style: computed(move || {
                                                Css::new()
                                                    .height(px(6))
                                                    .width(percent(pct_val.get().clamp(0.0, 100.0) as f32))
                                                    .border_radius(px(3))
                                                    .background_color(Color::hex(cat.get().color_hex()))
                                            }),
                                        ) {
                                        }
                                    }
                                }
                            }
                        },
                    )
                }
            }
        }
    }
}
