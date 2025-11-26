pub struct TabViewer {}

impl egui_dock::TabViewer for TabViewer {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) {
        match tab {
            Tab::Plot
        }
    };
}

pub enum Tab {
    PlotWindow(),
    LogConsole()
}

impl Tab {
    pub fn plot_window() -> Self {
        Self::PlotWindow()
    }
}
