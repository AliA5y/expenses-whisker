use whisker::css::{AlignItems, Color, Cursor, FontWeight, JustifyContent};
use whisker::prelude::*;
use whisker_icons::{Icon, lucide};

use crate::models::{TimeframeFilter, Transaction, TransactionType, TypeFilter};
use crate::store::AppState;
use crate::theme::{
    self, ACCENT_EXPENSE, ACCENT_INCOME, ACCENT_PRIMARY, BG_CARD, BG_CARD_SURFACE, TEXT_MUTED,
    TEXT_PRIMARY, TEXT_SECONDARY,
};

#[component]
pub fn transactions_section(state: AppState) -> Element {
    let filtered = computed(move || state.filtered_transactions());
    let count_text = computed(move || format!("{} transactions", filtered.get().len()));
    let has_transactions = computed(move || !filtered.get().is_empty());

    let active_cat = state.selected_category;
    let clear_cat_tap = move |_| {
        active_cat.set(None);
    };

    render! {
        View(style: theme::card_style().gap(px(14))) {
            // Header & Filter Bar
            View(
                style: theme::column()
                    .gap(px(10))
                    .padding_bottom(px(12))
                    .border_bottom(theme::subtle_border()),
            ) {
                View(
                    style: theme::row()
                        .justify_content(JustifyContent::SpaceBetween)
                        .align_items(AlignItems::Center),
                ) {
                    View(style: theme::row().gap(px(8)).align_items(AlignItems::Center)) {
                        Text(
                            value: "Transactions",
                            style: Css::new()
                                .color(Color::hex(TEXT_PRIMARY))
                                .font_size(px(17))
                                .font_weight(FontWeight::Numeric(600)),
                        )
                        Text(
                            value: count_text,
                            style: Css::new().color(Color::hex(TEXT_MUTED)).font_size(px(12)),
                        )
                    }
                }

                // Filter Controls (Timeframe)
                View(style: theme::row().gap(px(8))) {
                    FilterPill(
                        label: "All",
                        is_active: computed(move || state.timeframe_filter.get() == TimeframeFilter::All),
                        on_tap: Callback::new(move |_| state.timeframe_filter.set(TimeframeFilter::All)),
                    )
                    FilterPill(
                        label: "This Month",
                        is_active: computed(move || state.timeframe_filter.get() == TimeframeFilter::ThisMonth),
                        on_tap: Callback::new(move |_| state.timeframe_filter.set(TimeframeFilter::ThisMonth)),
                    )
                    FilterPill(
                        label: "Today",
                        is_active: computed(move || state.timeframe_filter.get() == TimeframeFilter::Today),
                        on_tap: Callback::new(move |_| state.timeframe_filter.set(TimeframeFilter::Today)),
                    )
                }
            }

            // Secondary filters: Type filter & Category reset
            View(
                style: theme::row()
                    .justify_content(JustifyContent::SpaceBetween)
                    .align_items(AlignItems::Center),
            ) {
                View(style: theme::row().gap(px(8))) {
                    TypePill(
                        label: "All Types",
                        is_active: computed(move || state.type_filter.get() == TypeFilter::All),
                        on_tap: Callback::new(move |_| state.type_filter.set(TypeFilter::All)),
                    )
                    TypePill(
                        label: "Expenses",
                        is_active: computed(move || state.type_filter.get() == TypeFilter::ExpenseOnly),
                        on_tap: Callback::new(move |_| state.type_filter.set(TypeFilter::ExpenseOnly)),
                    )
                    TypePill(
                        label: "Income",
                        is_active: computed(move || state.type_filter.get() == TypeFilter::IncomeOnly),
                        on_tap: Callback::new(move |_| state.type_filter.set(TypeFilter::IncomeOnly)),
                    )
                }

                Show(
                    when: move || active_cat.get().is_some(),
                ) {
                    View(
                        style: theme::row()
                            .gap(px(6))
                            .padding_left(px(10))
                            .padding_right(px(10))
                            .padding_top(px(4))
                            .padding_bottom(px(4))
                            .border_radius(px(8))
                            .background_color(Color::hex(0x232938))
                            .cursor(Cursor::Pointer),
                        on_tap: clear_cat_tap,
                    ) {
                        Text(
                            value: computed(move || {
                                if let Some(c) = active_cat.get() {
                                    format!("Category: {}", c.display_name())
                                } else {
                                    String::new()
                                }
                            }),
                            style: Css::new()
                                .color(Color::hex(ACCENT_PRIMARY))
                                .font_size(px(12)),
                        )
                        Icon(svg: lucide::X, color: "#6366f1", size: "12")
                    }
                }
            }

            // Transactions list or empty state
            Show(
                when: move || has_transactions.get(),
                fallback: move || render! {
                    View(
                        style: theme::column()
                            .align_items(AlignItems::Center)
                            .justify_content(JustifyContent::Center)
                            .padding_top(px(40))
                            .padding_bottom(px(40))
                            .gap(px(12)),
                    ) {
                        View(
                            style: theme::row()
                                .justify_content(JustifyContent::Center)
                                .align_items(AlignItems::Center)
                                .width(px(48))
                                .height(px(48))
                                .border_radius(px(12))
                                .background_color(Color::hex(BG_CARD_SURFACE)),
                        ) {
                            Icon(svg: lucide::Inbox, color: "#64748b", size: "24")
                        }
                        Text(
                            value: "No transactions found",
                            style: Css::new()
                                .color(Color::hex(TEXT_PRIMARY))
                                .font_size(px(16))
                                .font_weight(FontWeight::Numeric(600)),
                        )
                        Text(
                            value: "Try adjusting your filters or record a new transaction",
                            style: Css::new().color(Color::hex(TEXT_MUTED)).font_size(px(13)),
                        )
                    }
                },
            ) {
                View(style: theme::column().gap(px(10))) {
                    ForEach(
                        each: move || filtered.get(),
                        key: |tx: &Transaction| tx.id.clone(),
                        children: move |tx_sig: ReadSignal<Transaction>| {
                            let tx = tx_sig.get();
                            let tx_id = tx.id.clone();
                            let is_income = tx.transaction_type == TransactionType::Income;
                            let amt_formatted = if is_income {
                                format!("+${:.2}", tx.amount)
                            } else {
                                format!("-${:.2}", tx.amount)
                            };
                            let amt_color = if is_income {
                                ACCENT_INCOME
                            } else {
                                ACCENT_EXPENSE
                            };
                            let cat_color = tx.category.color_hex();
                            let cat_bg = tx.category.bg_color_hex();
                            let cat_icon = tx.category.icon_svg();
                            let cat_name = tx.category.display_name().to_string();
                            let date_and_note = if tx.note.is_empty() {
                                tx.date.clone()
                            } else {
                                format!("{} ({})", tx.date, tx.note)
                            };
                        
                            let delete_tap = {
                                let id_clone = tx_id.clone();
                                move |_| {
                                    state.delete_transaction(&id_clone);
                                }
                            };
                        
                            render! {
                                View(
                                    style: theme::row()
                                        .justify_content(JustifyContent::SpaceBetween)
                                        .align_items(AlignItems::Center)
                                        .padding(px(14))
                                        .border_radius(px(12))
                                        .background_color(Color::hex(0x13161f))
                                        .border(theme::subtle_border()),
                                ) {
                                    // Left: Category icon & details
                                    View(style: theme::row().gap(px(14))) {
                                        View(
                                            style: theme::row()
                                                .justify_content(JustifyContent::Center)
                                                .align_items(AlignItems::Center)
                                                .width(px(40))
                                                .height(px(40))
                                                .border_radius(px(10))
                                                .background_color(Color::hex(cat_bg)),
                                        ) {
                                            Icon(svg: cat_icon, color: "#ffffff", size: "20")
                                        }
                        
                                        View(style: theme::column().gap(px(2))) {
                                            Text(
                                                value: tx.title.clone(),
                                                style: Css::new()
                                                    .color(Color::hex(TEXT_PRIMARY))
                                                    .font_size(px(15))
                                                    .font_weight(FontWeight::Numeric(600)),
                                            )
                                            View(style: theme::row().gap(px(8))) {
                                                Text(
                                                    value: cat_name,
                                                    style: Css::new()
                                                        .color(Color::hex(cat_color))
                                                        .font_size(px(12))
                                                        .font_weight(FontWeight::Numeric(500)),
                                                )
                                                Text(
                                                    value: "•",
                                                    style: Css::new().color(Color::hex(TEXT_MUTED)).font_size(px(12)),
                                                )
                                                Text(
                                                    value: date_and_note,
                                                    style: Css::new().color(Color::hex(TEXT_MUTED)).font_size(px(12)),
                                                )
                                            }
                                        }
                                    }
                        
                                    // Right: Amount & Delete Button
                                    View(style: theme::row().gap(px(14))) {
                                        Text(
                                            value: amt_formatted,
                                            style: Css::new()
                                                .color(Color::hex(amt_color))
                                                .font_size(px(16))
                                                .font_weight(FontWeight::Numeric(700)),
                                        )
                                        View(
                                            style: theme::row()
                                                .justify_content(JustifyContent::Center)
                                                .align_items(AlignItems::Center)
                                                .width(px(32))
                                                .height(px(32))
                                                .border_radius(px(8))
                                                .background_color(Color::hex(BG_CARD))
                                                .cursor(Cursor::Pointer),
                                            on_tap: delete_tap,
                                        ) {
                                            Icon(svg: lucide::Trash2, color: "#ef4444", size: "15")
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

#[component]
fn filter_pill(label: &'static str, is_active: Signal<bool>, on_tap: Callback) -> Element {
    let tap = move |_| on_tap.call();
    render! {
        View(
            style: computed(move || {
                let active = is_active.get();
                theme::row()
                    .padding_left(px(12))
                    .padding_right(px(12))
                    .padding_top(px(6))
                    .padding_bottom(px(6))
                    .border_radius(px(8))
                    .cursor(Cursor::Pointer)
                    .background_color(if active {
                        Color::hex(ACCENT_PRIMARY)
                    } else {
                        Color::hex(BG_CARD_SURFACE)
                    })
            }),
            on_tap: tap,
        ) {
            Text(
                value: label,
                style: computed(move || {
                    let active = is_active.get();
                    Css::new()
                        .color(if active {
                            Color::hex(0xffffff)
                        } else {
                            Color::hex(TEXT_SECONDARY)
                        })
                        .font_size(px(13))
                        .font_weight(if active {
                            FontWeight::Numeric(600)
                        } else {
                            FontWeight::Numeric(400)
                        })
                }),
            )
        }
    }
}

#[component]
fn type_pill(label: &'static str, is_active: Signal<bool>, on_tap: Callback) -> Element {
    let tap = move |_| on_tap.call();
    render! {
        View(
            style: computed(move || {
                let active = is_active.get();
                theme::row()
                    .padding_left(px(10))
                    .padding_right(px(10))
                    .padding_top(px(4))
                    .padding_bottom(px(4))
                    .border_radius(px(6))
                    .cursor(Cursor::Pointer)
                    .background_color(if active {
                        Color::hex(0x2d3448)
                    } else {
                        Color::Transparent
                    })
            }),
            on_tap: tap,
        ) {
            Text(
                value: label,
                style: computed(move || {
                    let active = is_active.get();
                    Css::new()
                        .color(if active {
                            Color::hex(TEXT_PRIMARY)
                        } else {
                            Color::hex(TEXT_MUTED)
                        })
                        .font_size(px(12))
                        .font_weight(if active {
                            FontWeight::Numeric(600)
                        } else {
                            FontWeight::Numeric(400)
                        })
                }),
            )
        }
    }
}
