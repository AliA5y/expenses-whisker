//! ExpenseFlow — Ready-to-use Expense Tracker built with Whisker.

pub mod models;
pub mod store;
pub mod theme;
pub mod ui;

use whisker::css::{AlignItems, Color};
use whisker::prelude::*;
use whisker::runtime::view::Element;

use store::AppState;
use theme::{BG_CANVAS, column};
use ui::{AddTransactionModal, AppHeader, CategoryAnalytics, SummarySection, TransactionsSection};

#[whisker::main]
pub fn app() -> Element {
    let state = AppState::new();

    let page_style = column()
        .background_color(Color::hex(BG_CANVAS))
        .flex_grow(1.0)
        .flex_shrink(1.0)
        .min_height(px(0))
        .align_items(AlignItems::Center);

    let container_style = column()
        .width(percent(100.0))
        .max_width(px(600))
        .padding_left(px(16))
        .padding_right(px(16))
        .padding_top(px(20))
        .padding_bottom(px(40))
        .gap(px(18));

    let content_stack_style = column().gap(px(18));

    render! {
        ScrollView(style: page_style) {
            View(style: container_style) {
                AppHeader(state: state)
                SummarySection(state: state)
                AddTransactionModal(state: state)

                View(style: content_stack_style) {
                    TransactionsSection(state: state)
                    CategoryAnalytics(state: state)
                }
            }
        }
    }
}
