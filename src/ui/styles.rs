use eframe::egui;

#[allow(dead_code)]
pub struct AppStyle;

impl AppStyle {
    #[allow(dead_code)]
    pub fn color_success() -> egui::Color32 {
        egui::Color32::GREEN
    }

    #[allow(dead_code)]
    pub fn color_error() -> egui::Color32 {
        egui::Color32::RED
    }

    #[allow(dead_code)]
    pub fn color_warning() -> egui::Color32 {
        egui::Color32::YELLOW
    }

    #[allow(dead_code)]
    pub fn color_info() -> egui::Color32 {
        egui::Color32::LIGHT_BLUE
    }

    #[allow(dead_code)]
    pub fn color_highlight() -> egui::Color32 {
        egui::Color32::from_rgb(255, 165, 0)
    }

    #[allow(dead_code)]
    pub fn text_success(message: &str) -> egui::RichText {
        egui::RichText::new(format!("✔{}", message))
            .color(Self::color_success())
            .size(12.0)
    }

    #[allow(dead_code)]
    pub fn text_error(message: &str) -> egui::RichText {
        egui::RichText::new(format!("❌{}", message))
            .color(Self::color_error())
            .size(12.0)
    }

    #[allow(dead_code)]
    pub fn text_warning(message: &str) -> egui::RichText {
        egui::RichText::new(format!("⚠️-{}", message))
            .color(Self::color_warning())
            .size(12.0)
    }

    #[allow(dead_code)]
    pub fn text_info(message: &str) -> egui::RichText {
        egui::RichText::new(format!("ℹ️-{}", message))
            .color(Self::color_info())
            .size(12.0)
    }
}

