slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window: MainWindow = MainWindow::new()?;

    main_window.on_request_increase_value({
        let ui_handle = main_window.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    main_window.run()
}
