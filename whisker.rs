fn main() {
    whisker_cng::run(|app| {
        app.name("ExpenseFlow")
            .bundle_id("rs.whisker.expense")
            .version("0.1.0")
            .build_number(1);
        app.ios(|ios| {
            ios.scheme("ExpenseFlow").deployment_target("15.0");
        });
        app.android(|android| {
            android
                .package("rs.whisker.expense")
                .application_id("rs.whisker.expense");
        });
    });
}
