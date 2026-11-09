use whisker::prelude::*;
use whisker_local_store::WhiskerLocalStore;

use crate::models::{Category, TimeframeFilter, Transaction, TransactionType, TypeFilter};

const STORAGE_KEY: &str = "whisker_expenses_v1";

#[derive(Clone, Copy)]
pub struct AppState {
    pub transactions: RwSignal<Vec<Transaction>>,
    pub timeframe_filter: RwSignal<TimeframeFilter>,
    pub type_filter: RwSignal<TypeFilter>,
    pub selected_category: RwSignal<Option<Category>>,
    pub is_add_modal_open: RwSignal<bool>,
    pub monthly_budget: RwSignal<f64>,
}

impl AppState {
    pub fn new() -> Self {
        let initial_list = load_transactions();
        let state = Self {
            transactions: signal(initial_list),
            timeframe_filter: signal(TimeframeFilter::All),
            type_filter: signal(TypeFilter::All),
            selected_category: signal(None),
            is_add_modal_open: signal(false),
            monthly_budget: signal(2000.0),
        };

        // Reactive effect to persist transactions on every change
        let txs = state.transactions;
        effect(move || {
            let current = txs.get();
            persist_transactions(&current);
        });

        state
    }

    pub fn add_transaction(&self, tx: Transaction) {
        self.transactions.update(|list| {
            list.insert(0, tx);
        });
    }

    pub fn delete_transaction(&self, id: &str) {
        let target_id = id.to_string();
        self.transactions.update(|list| {
            list.retain(|t| t.id != target_id);
        });
    }

    pub fn reset_defaults(&self) {
        self.transactions.set(sample_transactions());
    }

    pub fn clear_all(&self) {
        self.transactions.set(Vec::new());
    }

    pub fn filtered_transactions(&self) -> Vec<Transaction> {
        let list = self.transactions.get();
        let tf = self.timeframe_filter.get();
        let ty = self.type_filter.get();
        let cat = self.selected_category.get();

        list.into_iter()
            .filter(|t| match ty {
                TypeFilter::All => true,
                TypeFilter::ExpenseOnly => t.transaction_type == TransactionType::Expense,
                TypeFilter::IncomeOnly => t.transaction_type == TransactionType::Income,
            })
            .filter(|t| match cat {
                Some(selected) => t.category == selected,
                None => true,
            })
            .filter(|t| match tf {
                TimeframeFilter::All => true,
                TimeframeFilter::ThisMonth => t.date.starts_with("2026-11"),
                TimeframeFilter::Today => t.date == "2026-11-09",
            })
            .collect()
    }

    pub fn total_income(&self) -> f64 {
        self.transactions
            .get()
            .iter()
            .filter(|t| t.transaction_type == TransactionType::Income)
            .map(|t| t.amount)
            .sum()
    }

    pub fn total_expense(&self) -> f64 {
        self.transactions
            .get()
            .iter()
            .filter(|t| t.transaction_type == TransactionType::Expense)
            .map(|t| t.amount)
            .sum()
    }

    pub fn balance(&self) -> f64 {
        self.total_income() - self.total_expense()
    }

    pub fn category_spending(&self) -> Vec<(Category, f64, f64)> {
        let list = self.transactions.get();
        let total_exp = self.total_expense();
        let mut results = Vec::new();

        for &category in Category::all() {
            let sum: f64 = list
                .iter()
                .filter(|t| {
                    t.transaction_type == TransactionType::Expense && t.category == category
                })
                .map(|t| t.amount)
                .sum();

            if sum > 0.0 {
                let percent = if total_exp > 0.0 {
                    (sum / total_exp) * 100.0
                } else {
                    0.0
                };
                results.push((category, sum, percent));
            }
        }

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results
    }
}

fn load_transactions() -> Vec<Transaction> {
    if let Ok(Some(raw)) = WhiskerLocalStore::load(STORAGE_KEY.into()) {
        if let Ok(parsed) = serde_json::from_str::<Vec<Transaction>>(&raw) {
            if !parsed.is_empty() {
                return parsed;
            }
        }
    }
    sample_transactions()
}

fn persist_transactions(transactions: &[Transaction]) {
    if let Ok(json) = serde_json::to_string(transactions) {
        let _ = WhiskerLocalStore::save(STORAGE_KEY.into(), json.into());
    }
}

pub fn sample_transactions() -> Vec<Transaction> {
    vec![
        Transaction {
            id: "tx-1".into(),
            title: "Monthly Salary".into(),
            amount: 4500.0,
            category: Category::Salary,
            transaction_type: TransactionType::Income,
            date: "2026-11-01".into(),
            note: "Direct deposit from company".into(),
        },
        Transaction {
            id: "tx-2".into(),
            title: "Supermarket Groceries".into(),
            amount: 124.50,
            category: Category::Food,
            transaction_type: TransactionType::Expense,
            date: "2026-11-09".into(),
            note: "Weekly fresh groceries & supplies".into(),
        },
        Transaction {
            id: "tx-3".into(),
            title: "Freelance UI Project".into(),
            amount: 750.0,
            category: Category::Salary,
            transaction_type: TransactionType::Income,
            date: "2026-11-05".into(),
            note: "Design sprint payment".into(),
        },
        Transaction {
            id: "tx-4".into(),
            title: "Metro Monthly Pass".into(),
            amount: 48.0,
            category: Category::Transport,
            transaction_type: TransactionType::Expense,
            date: "2026-11-04".into(),
            note: "City transit card renewal".into(),
        },
        Transaction {
            id: "tx-5".into(),
            title: "Fiber Internet Bill".into(),
            amount: 65.0,
            category: Category::Bills,
            transaction_type: TransactionType::Expense,
            date: "2026-11-03".into(),
            note: "Monthly broadband subscription".into(),
        },
        Transaction {
            id: "tx-6".into(),
            title: "Wireless Headphones".into(),
            amount: 179.99,
            category: Category::Shopping,
            transaction_type: TransactionType::Expense,
            date: "2026-11-02".into(),
            note: "Office tech gear upgrade".into(),
        },
        Transaction {
            id: "tx-7".into(),
            title: "Coffee & Pastries".into(),
            amount: 14.50,
            category: Category::Food,
            transaction_type: TransactionType::Expense,
            date: "2026-11-09".into(),
            note: "Morning team catch-up".into(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_transactions_balance() {
        let samples = sample_transactions();
        let total_income: f64 = samples
            .iter()
            .filter(|t| t.transaction_type == TransactionType::Income)
            .map(|t| t.amount)
            .sum();
        let total_expense: f64 = samples
            .iter()
            .filter(|t| t.transaction_type == TransactionType::Expense)
            .map(|t| t.amount)
            .sum();
        assert!(total_income > 0.0);
        assert!(total_expense > 0.0);
        assert!(total_income > total_expense);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let samples = sample_transactions();
        let json = serde_json::to_string(&samples).expect("serialize");
        let deserialized: Vec<Transaction> = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(samples.len(), deserialized.len());
        assert_eq!(samples[0].id, deserialized[0].id);
        assert_eq!(samples[0].amount, deserialized[0].amount);
    }

    #[test]
    fn test_category_metadata() {
        for &cat in Category::all() {
            assert!(!cat.display_name().is_empty());
            assert!(!cat.icon_svg().is_empty());
            assert!(cat.color_hex() > 0);
            assert!(cat.bg_color_hex() > 0);
        }
    }
}
