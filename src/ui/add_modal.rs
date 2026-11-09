use whisker::css::{AlignItems, Color, Cursor, FlexWrap, FontWeight, JustifyContent};
use whisker::prelude::*;
use whisker_icons::{Icon, lucide};
use whisker_input::{Input, KeyboardType};

use crate::models::{Category, Transaction, TransactionType};
use crate::store::AppState;
use crate::theme::{
    self, ACCENT_EXPENSE, ACCENT_INCOME, ACCENT_PRIMARY, BG_CARD_SURFACE, BG_INPUT, TEXT_MUTED,
    TEXT_PRIMARY, TEXT_SECONDARY,
};

#[component]
pub fn add_transaction_modal(state: AppState) -> Element {
    let amount_input = RwSignal::new(String::new());
    let title_input = RwSignal::new(String::new());
    let note_input = RwSignal::new(String::new());
    let selected_type = RwSignal::new(TransactionType::Expense);
    let selected_category = RwSignal::new(Category::Food);
    let error_text = RwSignal::new(String::new());

    let close_modal = move |_| {
        error_text.set(String::new());
        state.is_add_modal_open.set(false);
    };

    let save_transaction = move |_| {
        let title = title_input.get().trim().to_string();
        if title.is_empty() {
            error_text.set("Please enter a title or merchant description".into());
            return;
        }

        let raw_amt = amount_input.get().trim().replace('$', "").to_string();
        let amount = match raw_amt.parse::<f64>() {
            Ok(v) if v > 0.0 => v,
            _ => {
                error_text.set("Please enter a valid amount greater than $0.00".into());
                return;
            }
        };

        let note = note_input.get().trim().to_string();
        let cat = selected_category.get();
        let ty = selected_type.get();

        // Unique ID based on transaction count and millis
        let tx_id = format!("tx-{}", (state.transactions.get().len() + 1) * 1000 + 42);

        state.add_transaction(Transaction {
            id: tx_id,
            title,
            amount,
            category: cat,
            transaction_type: ty,
            date: "2026-11-09".into(),
            note,
        });

        // Reset fields and close
        amount_input.set(String::new());
        title_input.set(String::new());
        note_input.set(String::new());
        error_text.set(String::new());
        state.is_add_modal_open.set(false);
    };

    render! {
        Show(
            when: move || state.is_add_modal_open.get(),
        ) {
            View(
                style: theme::column()
                    .padding(px(24))
                    .border_radius(px(20))
                    .background_color(Color::hex(0x131622))
                    .border(theme::subtle_border())
                    .gap(px(20)),
            ) {
                // Modal Header
                View(
                    style: theme::row()
                        .justify_content(JustifyContent::SpaceBetween)
                        .align_items(AlignItems::Center),
                ) {
                    View(style: theme::row().gap(px(10))) {
                        View(
                            style: theme::row()
                                .justify_content(JustifyContent::Center)
                                .align_items(AlignItems::Center)
                                .width(px(36))
                                .height(px(36))
                                .border_radius(px(10))
                                .background_color(Color::hex(ACCENT_PRIMARY)),
                        ) {
                            Icon(svg: lucide::Plus, color: "#ffffff", size: "20")
                        }
                        Text(
                            value: "New Transaction",
                            style: Css::new()
                                .color(Color::hex(TEXT_PRIMARY))
                                .font_size(px(18))
                                .font_weight(FontWeight::Numeric(700)),
                        )
                    }

                    View(
                        style: theme::row()
                            .justify_content(JustifyContent::Center)
                            .align_items(AlignItems::Center)
                            .width(px(32))
                            .height(px(32))
                            .border_radius(px(8))
                            .background_color(Color::hex(BG_CARD_SURFACE))
                            .cursor(Cursor::Pointer),
                        on_tap: close_modal,
                    ) {
                        Icon(svg: lucide::X, color: "#94a3b8", size: "16")
                    }
                }

                // Error Banner (if error)
                Show(
                    when: move || !error_text.get().is_empty(),
                ) {
                    View(
                        style: theme::row()
                            .padding(px(12))
                            .border_radius(px(10))
                            .background_color(Color::hex(0x381216))
                            .gap(px(8)),
                    ) {
                        Text(
                            value: computed(move || error_text.get()),
                            style: Css::new()
                                .color(Color::hex(ACCENT_EXPENSE))
                                .font_size(px(13)),
                        )
                    }
                }

                // Type Toggle: Expense vs Income
                View(
                    style: theme::row()
                        .padding(px(4))
                        .border_radius(px(12))
                        .background_color(Color::hex(BG_CARD_SURFACE))
                        .gap(px(6)),
                ) {
                    // Expense Tab
                    View(
                        style: computed(move || {
                            let is_exp = selected_type.get() == TransactionType::Expense;
                            theme::row()
                                .flex_grow(1.0)
                                .justify_content(JustifyContent::Center)
                                .align_items(AlignItems::Center)
                                .padding_top(px(10))
                                .padding_bottom(px(10))
                                .border_radius(px(8))
                                .cursor(Cursor::Pointer)
                                .background_color(if is_exp {
                                    Color::hex(ACCENT_EXPENSE)
                                } else {
                                    Color::Transparent
                                })
                        }),
                        on_tap: move |_| {
                            selected_type.set(TransactionType::Expense);
                            selected_category.set(Category::Food);
                        },
                    ) {
                        Text(
                            value: "Expense",
                            style: Css::new()
                                .color(Color::hex(0xffffff))
                                .font_size(px(14))
                                .font_weight(FontWeight::Numeric(600)),
                        )
                    }

                    // Income Tab
                    View(
                        style: computed(move || {
                            let is_inc = selected_type.get() == TransactionType::Income;
                            theme::row()
                                .flex_grow(1.0)
                                .justify_content(JustifyContent::Center)
                                .align_items(AlignItems::Center)
                                .padding_top(px(10))
                                .padding_bottom(px(10))
                                .border_radius(px(8))
                                .cursor(Cursor::Pointer)
                                .background_color(if is_inc {
                                    Color::hex(ACCENT_INCOME)
                                } else {
                                    Color::Transparent
                                })
                        }),
                        on_tap: move |_| {
                            selected_type.set(TransactionType::Income);
                            selected_category.set(Category::Salary);
                        },
                    ) {
                        Text(
                            value: "Income",
                            style: Css::new()
                                .color(Color::hex(0xffffff))
                                .font_size(px(14))
                                .font_weight(FontWeight::Numeric(600)),
                        )
                    }
                }

                // Amount Field
                View(style: theme::column().gap(px(8))) {
                    Text(
                        value: "Amount ($)",
                        style: Css::new()
                            .color(Color::hex(TEXT_SECONDARY))
                            .font_size(px(13)),
                    )
                    Input(
                        text: amount_input,
                        placeholder: "0.00",
                        placeholder_color: "#64748b",
                        caret_color: "#6366f1",
                        keyboard_type: KeyboardType::Decimal,
                        style: Css::new()
                            .padding(px(12))
                            .border_radius(px(10))
                            .background_color(Color::hex(BG_INPUT))
                            .border(theme::subtle_border())
                            .color(Color::hex(TEXT_PRIMARY))
                            .font_size(px(18))
                            .font_weight(FontWeight::Numeric(600)),
                    )

                    // Quick Increment Buttons
                    View(style: theme::row().gap(px(8))) {
                        QuickAddButton(amount: 5.0, target: amount_input)
                        QuickAddButton(amount: 10.0, target: amount_input)
                        QuickAddButton(amount: 25.0, target: amount_input)
                        QuickAddButton(amount: 50.0, target: amount_input)
                        QuickAddButton(amount: 100.0, target: amount_input)
                    }
                }

                // Category Selector
                View(style: theme::column().gap(px(8))) {
                    Text(
                        value: "Category",
                        style: Css::new()
                            .color(Color::hex(TEXT_SECONDARY))
                            .font_size(px(13)),
                    )
                    View(
                        style: theme::row().flex_wrap(FlexWrap::Wrap).gap(px(8)),
                    ) {
                        ForEach(
                            each: move || Category::all().to_vec(),
                            key: |cat: &Category| *cat,
                            children: move |cat_sig: ReadSignal<Category>| {
                                let c = cat_sig.get();
                                let is_sel = computed(move || selected_category.get() == c);
                                let tap = move |_| selected_category.set(c);
                            
                                render! {
                                    View(
                                        style: computed(move || {
                                            let active = is_sel.get();
                                            theme::row()
                                                .gap(px(6))
                                                .padding_left(px(12))
                                                .padding_right(px(12))
                                                .padding_top(px(8))
                                                .padding_bottom(px(8))
                                                .border_radius(px(8))
                                                .cursor(Cursor::Pointer)
                                                .background_color(if active {
                                                    Color::hex(c.color_hex())
                                                } else {
                                                    Color::hex(BG_CARD_SURFACE)
                                                })
                                        }),
                                        on_tap: tap,
                                    ) {
                                        Icon(svg: c.icon_svg(), color: "#ffffff", size: "14")
                                        Text(
                                            value: c.display_name(),
                                            style: computed(move || {
                                                let active = is_sel.get();
                                                Css::new()
                                                    .color(if active {
                                                        Color::hex(0xffffff)
                                                    } else {
                                                        Color::hex(TEXT_SECONDARY)
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
                            },
                        )
                    }
                }

                // Title / Description
                View(style: theme::column().gap(px(8))) {
                    Text(
                        value: "Title / Merchant",
                        style: Css::new()
                            .color(Color::hex(TEXT_SECONDARY))
                            .font_size(px(13)),
                    )
                    Input(
                        text: title_input,
                        placeholder: "e.g. Whole Foods, Uber, Client payment…",
                        placeholder_color: "#64748b",
                        caret_color: "#6366f1",
                        style: Css::new()
                            .padding(px(12))
                            .border_radius(px(10))
                            .background_color(Color::hex(BG_INPUT))
                            .border(theme::subtle_border())
                            .color(Color::hex(TEXT_PRIMARY))
                            .font_size(px(14)),
                    )
                }

                // Note (optional)
                View(style: theme::column().gap(px(8))) {
                    Text(
                        value: "Note (Optional)",
                        style: Css::new()
                            .color(Color::hex(TEXT_SECONDARY))
                            .font_size(px(13)),
                    )
                    Input(
                        text: note_input,
                        placeholder: "Add details, tags, or invoice note…",
                        placeholder_color: "#64748b",
                        caret_color: "#6366f1",
                        style: Css::new()
                            .padding(px(12))
                            .border_radius(px(10))
                            .background_color(Color::hex(BG_INPUT))
                            .border(theme::subtle_border())
                            .color(Color::hex(TEXT_PRIMARY))
                            .font_size(px(14)),
                    )
                }

                // Action Buttons
                View(
                    style: theme::row()
                        .justify_content(JustifyContent::FlexEnd)
                        .gap(px(12))
                        .padding_top(px(8)),
                ) {
                    View(
                        style: theme::row()
                            .padding_left(px(16))
                            .padding_right(px(16))
                            .padding_top(px(10))
                            .padding_bottom(px(10))
                            .border_radius(px(10))
                            .background_color(Color::hex(BG_CARD_SURFACE))
                            .cursor(Cursor::Pointer),
                        on_tap: close_modal,
                    ) {
                        Text(
                            value: "Cancel",
                            style: Css::new()
                                .color(Color::hex(TEXT_SECONDARY))
                                .font_size(px(14)),
                        )
                    }

                    View(
                        style: theme::row()
                            .gap(px(8))
                            .padding_left(px(20))
                            .padding_right(px(20))
                            .padding_top(px(10))
                            .padding_bottom(px(10))
                            .border_radius(px(10))
                            .background_color(Color::hex(ACCENT_PRIMARY))
                            .cursor(Cursor::Pointer),
                        on_tap: save_transaction,
                    ) {
                        Icon(svg: lucide::Check, color: "#ffffff", size: "16")
                        Text(
                            value: "Save Transaction",
                            style: Css::new()
                                .color(Color::hex(0xffffff))
                                .font_size(px(14))
                                .font_weight(FontWeight::Numeric(600)),
                        )
                    }
                }
            }
        }
    }
}

#[component]
fn quick_add_button(amount: f64, target: RwSignal<String>) -> Element {
    let tap = move |_| {
        let cur = target
            .get()
            .trim()
            .replace('$', "")
            .parse::<f64>()
            .unwrap_or(0.0);
        target.set(format!("{:.2}", cur + amount));
    };

    render! {
        View(
            style: theme::row()
                .padding_left(px(10))
                .padding_right(px(10))
                .padding_top(px(4))
                .padding_bottom(px(4))
                .border_radius(px(6))
                .background_color(Color::hex(BG_CARD_SURFACE))
                .cursor(Cursor::Pointer),
            on_tap: tap,
        ) {
            Text(
                value: format!("+${:.0}", amount),
                style: Css::new().color(Color::hex(TEXT_MUTED)).font_size(px(11)),
            )
        }
    }
}
