use iced::widget::{button, container};
use iced::{Background, Border, Color, Theme};

#[derive(Debug, Clone)]
pub struct AppStyle {
    pub name: String,
    pub bg_color: Color,
    pub card_bg: Color,
    pub header_bg: Color,
    pub title_color: Color,
    pub text_color: Color,
    pub value_color: Color,
    pub cpu_color: Color,
    pub memory_color: Color,
    pub gpu_color: Color,
    pub gpu_memory_color: Color,
    pub button_bg: Color,
    pub button_hover: Color,
    pub danger_bg: Color,
    pub border_color: Color,
}

#[derive(Debug, Clone)]
struct CustomContainer {
    bg: Option<Color>,
    border: Border,
}

impl container::StyleSheet for CustomContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: self.bg.map(Background::Color),
            border: self.border.clone(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
struct CustomButton {
    bg: Color,
    hover_bg: Color,
    text: Color,
    radius: f32,
}

impl button::StyleSheet for CustomButton {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(self.bg)),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: self.radius.into(),
            },
            text_color: self.text,
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(self.hover_bg)),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: self.radius.into(),
            },
            text_color: self.text,
            ..Default::default()
        }
    }
}

impl AppStyle {
    pub fn button_theme(&self) -> iced::theme::Button {
        iced::theme::Button::Custom(Box::new(CustomButton {
            bg: self.button_bg,
            hover_bg: self.button_hover,
            text: Color::WHITE,
            radius: 8.0,
        }))
    }

    pub fn danger_button_theme(&self) -> iced::theme::Button {
        let hover = Color::from_rgba(
            (self.danger_bg.r + 0.1).min(1.0),
            (self.danger_bg.g + 0.1).min(1.0),
            (self.danger_bg.b + 0.1).min(1.0),
            1.0,
        );

        iced::theme::Button::Custom(Box::new(CustomButton {
            bg: self.danger_bg,
            hover_bg: hover,
            text: Color::WHITE,
            radius: 8.0,
        }))
    }

    pub fn container_theme(&self) -> iced::theme::Container {
        iced::theme::Container::Custom(Box::new(CustomContainer {
            bg: Some(self.bg_color),
            border: Border::default(),
        }))
    }

    pub fn card_theme(&self) -> iced::theme::Container {
        iced::theme::Container::Custom(Box::new(CustomContainer {
            bg: Some(self.card_bg),
            border: Border {
                color: self.border_color,
                width: 1.0,
                radius: 12.0.into(),
            },
        }))
    }

    pub fn header_theme(&self) -> iced::theme::Container {
        iced::theme::Container::Custom(Box::new(CustomContainer {
            bg: Some(self.header_bg),
            border: Border {
                color: self.border_color,
                width: 1.0,
                radius: 0.0.into(),
            },
        }))
    }
}

pub fn create_styles() -> Vec<AppStyle> {
    vec![
        AppStyle {
            name: "Dark".to_string(),
            bg_color: Color::from_rgb(0.1, 0.1, 0.12),
            card_bg: Color::from_rgba(0.15, 0.15, 0.18, 0.95),
            header_bg: Color::from_rgba(0.12, 0.12, 0.15, 0.98),
            title_color: Color::from_rgb(0.9, 0.9, 0.95),
            text_color: Color::from_rgba(0.7, 0.7, 0.75, 1.0),
            value_color: Color::WHITE,
            cpu_color: Color::from_rgb(0.3, 0.8, 0.4),
            memory_color: Color::from_rgb(0.3, 0.6, 1.0),
            gpu_color: Color::from_rgb(0.9, 0.6, 0.2),
            gpu_memory_color: Color::from_rgb(0.8, 0.4, 0.9),
            button_bg: Color::from_rgb(0.2, 0.5, 0.8),
            button_hover: Color::from_rgb(0.3, 0.6, 0.9),
            danger_bg: Color::from_rgb(0.8, 0.2, 0.2),
            border_color: Color::from_rgba(0.3, 0.3, 0.35, 0.5),
        },
        AppStyle {
            name: "Tech Blue".to_string(),
            bg_color: Color::from_rgb(0.05, 0.08, 0.15),
            card_bg: Color::from_rgba(0.08, 0.12, 0.2, 0.95),
            header_bg: Color::from_rgba(0.06, 0.1, 0.18, 0.98),
            title_color: Color::from_rgb(0.4, 0.8, 1.0),
            text_color: Color::from_rgba(0.6, 0.7, 0.8, 1.0),
            value_color: Color::from_rgb(0.5, 0.9, 1.0),
            cpu_color: Color::from_rgb(0.2, 0.9, 0.6),
            memory_color: Color::from_rgb(0.3, 0.7, 1.0),
            gpu_color: Color::from_rgb(0.0, 0.8, 0.9),
            gpu_memory_color: Color::from_rgb(0.5, 0.5, 1.0),
            button_bg: Color::from_rgb(0.1, 0.4, 0.7),
            button_hover: Color::from_rgb(0.2, 0.5, 0.8),
            danger_bg: Color::from_rgb(0.7, 0.15, 0.15),
            border_color: Color::from_rgba(0.2, 0.4, 0.6, 0.5),
        },
        AppStyle {
            name: "Cyberpunk".to_string(),
            bg_color: Color::from_rgb(0.08, 0.02, 0.12),
            card_bg: Color::from_rgba(0.12, 0.04, 0.18, 0.95),
            header_bg: Color::from_rgba(0.1, 0.03, 0.15, 0.98),
            title_color: Color::from_rgb(1.0, 0.2, 0.6),
            text_color: Color::from_rgba(0.8, 0.6, 0.8, 1.0),
            value_color: Color::from_rgb(1.0, 0.4, 0.8),
            cpu_color: Color::from_rgb(0.0, 1.0, 0.8),
            memory_color: Color::from_rgb(1.0, 0.0, 0.8),
            gpu_color: Color::from_rgb(1.0, 1.0, 0.0),
            gpu_memory_color: Color::from_rgb(0.8, 0.0, 1.0),
            button_bg: Color::from_rgb(0.8, 0.1, 0.4),
            button_hover: Color::from_rgb(0.9, 0.2, 0.5),
            danger_bg: Color::from_rgb(0.9, 0.1, 0.1),
            border_color: Color::from_rgba(0.6, 0.2, 0.6, 0.5),
        },
        AppStyle {
            name: "Minimal Light".to_string(),
            bg_color: Color::from_rgb(0.95, 0.95, 0.96),
            card_bg: Color::from_rgba(1.0, 1.0, 1.0, 0.95),
            header_bg: Color::from_rgba(0.98, 0.98, 0.99, 0.98),
            title_color: Color::from_rgb(0.1, 0.1, 0.15),
            text_color: Color::from_rgba(0.4, 0.4, 0.45, 1.0),
            value_color: Color::from_rgb(0.1, 0.1, 0.1),
            cpu_color: Color::from_rgb(0.2, 0.7, 0.3),
            memory_color: Color::from_rgb(0.2, 0.5, 0.8),
            gpu_color: Color::from_rgb(0.8, 0.5, 0.1),
            gpu_memory_color: Color::from_rgb(0.6, 0.3, 0.7),
            button_bg: Color::from_rgb(0.2, 0.5, 0.8),
            button_hover: Color::from_rgb(0.3, 0.6, 0.9),
            danger_bg: Color::from_rgb(0.8, 0.2, 0.2),
            border_color: Color::from_rgba(0.8, 0.8, 0.82, 0.8),
        },
    ]
}

