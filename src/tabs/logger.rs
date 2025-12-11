pub struct LogConsole;

impl LogConsole {
    pub fn id() -> &'static str {
        "log_console"
    }
    pub fn title() -> egui::WidgetText {
        "Registros".into()
    }
    pub fn ui(ui: &mut egui::Ui) {
        egui_logger::logger_ui()
            .log_levels([true, true, true, true, false])
            .enable_ctx_menu(false)
            .enable_regex(false)
            .enable_max_log_output(false)
            .enable_category("sepic_gui::app", true)
            .enable_category("sepic_gui::serialcomms", true)
            .show(ui);
    }
}
