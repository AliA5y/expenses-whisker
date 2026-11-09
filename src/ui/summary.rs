use whisker::css::{AlignItems, Color, FontWeight, JustifyContent, Overflow};
use whisker::prelude::*;
use whisker_icons::{Icon, lucide};

use crate::store::AppState;
use crate::theme::{
    self, ACCENT_EXPENSE, ACCENT_INCOME, ACCENT_WARNING, BG_CARD_SURFACE, TEXT_MUTED, TEXT_PRIMARY,
    TEXT_SECONDARY,
};

#[component]
pub fn summary_section(state: AppState) -> Element {
    let balance_text = computed(move || {
        let b = state.balance();
        if b >= 0.0 {
            format!("${:.2}", b)
        } else {
            format!("-${:.2}", b.abs())
        }
    });

    let income_text = computed(move || format!("+${:.2}", state.total_income()));

    let expense_text = computed(move || format!("-${:.2}", state.total_expense()));

    let budget_text = computed(move || {
        let spent = state.total_expense();
        let budget = state.monthly_budget.get();
        let pct = if budget > 0.0 {
            (spent / budget) * 100.0
        } else {
            0.0
        };
        format!("${:.2} of ${:.2} ({:.1}%)", spent, budget, pct)
    });

    let progress_width = computed(move || {
        let spent = state.total_expense();
        let budget = state.monthly_budget.get();
        let pct = if budget > 0.0 {
            (spent / budget) * 100.0
        } else {
            0.0
        };
        pct.clamp(0.0, 100.0) as f32
    });

    let progress_color = computed(move || {
        let spent = state.total_expense();
        let budget = state.monthly_budget.get();
        let pct = if budget > 0.0 {
            (spent / budget) * 100.0
        } else {
            0.0
        };
        if pct > 90.0 {
            ACCENT_EXPENSE
        } else if pct > 75.0 {
            ACCENT_WARNING
        } else {
            ACCENT_INCOME
        }
    });

    render! {
        View(style: theme::column().gap(px(14))) {
            // Featured Balance Card (Full Width)
            View(
                style: theme::card_style().gap(px(6)),
            ) {
                View(
                    style: theme::row()
                        .justify_content(JustifyContent::SpaceBetween)
                        .align_items(AlignItems::Center),
                ) {
                    Text(
                        value: "Total Balance",
                        style: Css::new()
                            .color(Color::hex(TEXT_SECONDARY))
                            .font_size(px(14))
                            .font_weight(FontWeight::Numeric(500)),
                    )
                    View(
                        style: theme::row()
                            .justify_content(JustifyContent::Center)
                            .align_items(AlignItems::Center)
                            .width(px(32))
                            .height(px(32))
                            .border_radius(px(8))
                            .background_color(Color::hex(0x232938)),
                    ) {
                        Icon(svg: lucide::Wallet, color: "#6366f1", size: "18")
                    }
                }
                Text(
                    value: balance_text,
                    style: Css::new()
                        .color(Color::hex(TEXT_PRIMARY))
                        .font_size(px(32))
                        .font_weight(FontWeight::Numeric(700)),
                )
                Text(
                    value: "Net liquid funds available",
                    style: Css::new().color(Color::hex(TEXT_MUTED)).font_size(px(12)),
                )
            }

            // Income and Expenses 2-Column Row
            View(style: theme::row().gap(px(12))) {
                // Total Income Card
                View(
                    style: theme::card_style()
                        .flex_grow(1.0)
                        .flex_basis(px(0))
                        .gap(px(6)),
                ) {
                    View(
                        style: theme::row()
                            .justify_content(JustifyContent::SpaceBetween)
                            .align_items(AlignItems::Center),
                    ) {
                        Text(
                            value: "Total Income",
                            style: Css::new()
                                .color(Color::hex(TEXT_SECONDARY))
                                .font_size(px(13)),
                        )
                        View(
                            style: theme::row()
                                .justify_content(JustifyContent::Center)
                                .align_items(AlignItems::Center)
                                .width(px(26))
                                .height(px(26))
                                .border_radius(px(6))
                                .background_color(Color::hex(0x0e2b18)),
                        ) {
                            Icon(svg: lucide::TrendingUp, color: "#10b981", size: "15")
                        }
                    }
                    Text(
                        value: income_text,
                        style: Css::new()
                            .color(Color::hex(ACCENT_INCOME))
                            .font_size(px(20))
                            .font_weight(FontWeight::Numeric(700)),
                    )
                    Text(
                        value: "Salary & gigs",
                        style: Css::new().color(Color::hex(TEXT_MUTED)).font_size(px(11)),
                    )
                }

                // Total Expenses Card
                View(
                    style: theme::card_style()
                        .flex_grow(1.0)
                        .flex_basis(px(0))
                        .gap(px(6)),
                ) {
                    View(
                        style: theme::row()
                            .justify_content(JustifyContent::SpaceBetween)
                            .align_items(AlignItems::Center),
                    ) {
                        Text(
                            value: "Total Expenses",
                            style: Css::new()
                                .color(Color::hex(TEXT_SECONDARY))
                                .font_size(px(13)),
                        )
                        View(
                            style: theme::row()
                                .justify_content(JustifyContent::Center)
                                .align_items(AlignItems::Center)
                                .width(px(26))
                                .height(px(26))
                                .border_radius(px(6))
                                .background_color(Color::hex(0x2e1214)),
                        ) {
                            Icon(svg: lucide::TrendingDown, color: "#f43f5e", size: "15")
                        }
                    }
                    Text(
                        value: expense_text,
                        style: Css::new()
                            .color(Color::hex(ACCENT_EXPENSE))
                            .font_size(px(20))
                            .font_weight(FontWeight::Numeric(700)),
                    )
                    Text(
                        value: "Bills & living",
                        style: Css::new().color(Color::hex(TEXT_MUTED)).font_size(px(11)),
                    )
                }
            }

            // Monthly Budget Progress Banner
            View(
                style: theme::card_style().gap(px(12)),
            ) {
                View(
                    style: theme::row()
                        .justify_content(JustifyContent::SpaceBetween)
                        .align_items(AlignItems::Center),
                ) {
                    View(style: theme::row().gap(px(8))) {
                        Icon(svg: lucide::CreditCard, color: "#94a3b8", size: "16")
                        Text(
                            value: "Monthly Budget Progress",
                            style: Css::new()
                                .color(Color::hex(TEXT_PRIMARY))
                                .font_size(px(14))
                                .font_weight(FontWeight::Numeric(600)),
                        )
                    }
                    Text(
                        value: budget_text,
                        style: Css::new()
                            .color(Color::hex(TEXT_SECONDARY))
                            .font_size(px(13)),
                    )
                }

                // Progress Bar Track
                View(
                    style: Css::new()
                        .display_flex()
                        .height(px(8))
                        .border_radius(px(4))
                        .background_color(Color::hex(BG_CARD_SURFACE))
                        .overflow(Overflow::Hidden),
                ) {
                    // Active Fill
                    View(
                        style: computed(move || {
                            Css::new()
                                .height(px(8))
                                .width(percent(progress_width.get()))
                                .border_radius(px(4))
                                .background_color(Color::hex(progress_color.get()))
                        }),
                    ) {
                    }
                }
            }
        }
    }
}
