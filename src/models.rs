use serde::{Deserialize, Serialize};
use whisker_icons::lucide;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    Expense,
    Income,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Category {
    Food,
    Transport,
    Shopping,
    Bills,
    Entertainment,
    Health,
    Salary,
    Other,
}

impl Category {
    pub fn all() -> &'static [Category] {
        &[
            Category::Food,
            Category::Transport,
            Category::Shopping,
            Category::Bills,
            Category::Entertainment,
            Category::Health,
            Category::Salary,
            Category::Other,
        ]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Food => "Food & Dining",
            Self::Transport => "Transport",
            Self::Shopping => "Shopping",
            Self::Bills => "Bills & Utilities",
            Self::Entertainment => "Entertainment",
            Self::Health => "Health & Fitness",
            Self::Salary => "Salary & Income",
            Self::Other => "Other",
        }
    }

    pub fn icon_svg(&self) -> &'static str {
        match self {
            Self::Food => lucide::Coffee,
            Self::Transport => lucide::Car,
            Self::Shopping => lucide::ShoppingCart,
            Self::Bills => lucide::CreditCard,
            Self::Entertainment => lucide::TicketPlus,
            Self::Health => lucide::HeartPlus,
            Self::Salary => lucide::DollarSign,
            Self::Other => lucide::Box,
        }
    }

    pub fn color_hex(&self) -> u32 {
        match self {
            Self::Food => 0xf97316,          // Orange
            Self::Transport => 0x06b6d4,     // Cyan
            Self::Shopping => 0xa855f7,      // Purple
            Self::Bills => 0xef4444,         // Red
            Self::Entertainment => 0xec4899, // Pink
            Self::Health => 0x10b981,        // Emerald
            Self::Salary => 0x22c55e,        // Green
            Self::Other => 0x64748b,         // Slate
        }
    }

    pub fn bg_color_hex(&self) -> u32 {
        match self {
            Self::Food => 0x2d1b0f,
            Self::Transport => 0x0d2830,
            Self::Shopping => 0x261438,
            Self::Bills => 0x2e1214,
            Self::Entertainment => 0x2f1020,
            Self::Health => 0x0c271e,
            Self::Salary => 0x0e2b18,
            Self::Other => 0x1e2430,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub title: String,
    pub amount: f64,
    pub category: Category,
    pub transaction_type: TransactionType,
    pub date: String,
    pub note: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeframeFilter {
    All,
    ThisMonth,
    Today,
}

impl TimeframeFilter {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::All => "All Time",
            Self::ThisMonth => "This Month",
            Self::Today => "Today",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeFilter {
    All,
    ExpenseOnly,
    IncomeOnly,
}

impl TypeFilter {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::All => "All",
            Self::ExpenseOnly => "Expenses",
            Self::IncomeOnly => "Income",
        }
    }
}

