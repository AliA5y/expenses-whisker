# ExpenseFlow — Whisker Mobile Expense Tracker

A modern, fast, ready-to-use cross-platform Personal Finance & Expense Tracker built with [Whisker](https://whisker.rs) in Rust.

Whisker leverages Rust's fine-grained reactive runtime, Taffy flexbox layout engine, and native Android / iOS Host renderers without a virtual DOM or WebView.

---

## Features

- **Mobile-First Portrait Financial Dashboard**:
  - **Featured Total Balance Card**: Real-time reactive calculation of net liquid funds.
  - **Total Income & Expense Summary**: Side-by-side color-coded cards with directional trend indicators (`+$5,250.00` / `-$531.99`).
  - **Monthly Budget Progress Tracker**: Dynamic progress bar with automatic status thresholds (Healthy green, Warning yellow at >75%, Exceeded red at >90%).
- **Interactive Transaction Management**:
  - **Add Transaction Modal**: Fast modal with quick preset amount pills (`+$5`, `+$10`, `+$25`, `+$50`, `+$100`), Expense vs. Income type switcher, merchant title input, and optional notes.
  - **Category Selection**: 8 distinct categories with custom colored Lucide icon badges:
    - ☕ Food & Dining (`#f97316`)
    - 🚗 Transport (`#06b6d4`)
    - 🛒 Shopping (`#ec4899`)
    - 💳 Bills & Utilities (`#eab308`)
    - 🎟️ Entertainment (`#8b5cf6`)
    - ❤️ Health & Fitness (`#10b981`)
    - 💵 Salary & Income (`#22c55e`)
    - 📦 Other (`#64748b`)
  - **Delete & Manage**: One-tap delete for any transaction with automatic balance recalculation.
  - **Quick Sample Reset**: Easily reload realistic starter transactions or test customized data.
- **Smart Filtering**:
  - **Timeframe Filters**: All Time, This Month, Today.
  - **Type Filters**: All, Expenses Only, Income Only.
  - **Category Filtering**: Click any category in the analytics breakdown to instantly filter the list.
- **Spending Analytics**:
  - Real-time category breakdown with calculated percentage bars and totals.
- **Local Persistence**:
  - Automatically persists your transactions across launches using `WhiskerLocalStore`.

---

## Project Structure

```
expense/
├── Cargo.toml          # Workspace & package manifest with Whisker dependencies
├── whisker.rs          # App metadata & native Android configuration generator
├── README.md           # Documentation & instructions
├── .github/workflows/
│   └── ios.yml         # GitHub Actions CI for automated cloud iOS builds
└── src/
    ├── lib.rs          # Application entry point (#[whisker::main]) with mobile layout
    ├── models.rs       # Data models (Transaction, Category, TimeframeFilter, TypeFilter)
    ├── store.rs        # Reactive AppState & local storage persistence
    ├── theme.rs        # Design tokens, color palette, mobile card styling
    └── ui/
        ├── mod.rs
        ├── header.rs        # Header bar with wallet branding, Reset & Add buttons
        ├── summary.rs       # Stat cards & monthly budget progress bar
        ├── analytics.rs     # Category spending breakdown & progress bars
        ├── transactions.rs  # Filterable transaction list & empty states
        └── add_modal.rs     # Transaction creation modal with quick amount pills
```

---

## Running on Android

### 1. Requirements & Environment
Ensure the following environment variables are set in PowerShell:
```powershell
$env:ANDROID_HOME = "C:\Users\dev73\AppData\Local\Android\Sdk"
$env:JAVA_HOME = "C:\Program Files\Java\jdk-21"
$env:WHISKER_CLI = "C:\Users\dev73\.cargo\bin\whisker.exe"
$env:PATH = "C:\Users\dev73\AppData\Local\Android\Sdk\platform-tools;C:\Users\dev73\.cargo\bin;$env:PATH"
```

### 2. Verify Connected Device
Verify that your Android phone is detected and authorized via ADB:
```powershell
adb devices -l
# Output: b85c9160 device product:CPH2515 model:CPH2515
```

### 3. Build & Run
Run the app directly on your connected device:
```powershell
whisker run android --no-hot-patch --no-tui
```
This will:
1. Compile the Rust library for `aarch64-linux-android`.
2. Stage `libwhisker_expense.so` into the Android project.
3. Assemble the Android APK (`app-debug.apk`) via Gradle.
4. Install the package `rs.whisker.expense` onto device `CPH2515`.
5. Launch `rs.whisker.expense/.MainActivity`.

---

## Running on iOS

Because Apple requires Xcode, the iOS SDK, and Apple's Code Signing (`codesign`), iOS compilation must run on macOS:

1. **Option 1: Using a Mac**
   Transfer this project folder to any Mac with Xcode installed, connect your iPhone via USB, and run:
   ```bash
   whisker run ios
   ```
2. **Option 2: Using GitHub Actions (No Mac hardware required)**
   Push this repository to GitHub. The included workflow `.github/workflows/ios.yml` runs on `macos-latest` to build the Xcode project and output an installable artifact.

---

## Testing & Quality Assurance

Run the test suite:
```powershell
cargo test
```
All unit tests verify:
- Category metadata and icon mappings
- Sample transaction balance and math computations
- JSON serialization and deserialization roundtrips
