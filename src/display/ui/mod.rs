mod styles;
mod dashboard;

use iced::{
    widget::{column, container, row, text, button, text_input, horizontal_space, vertical_space, scrollable},
    Length, Color, Element, Alignment,
};
use crate::shared::SystemData;
use crate::Message;

pub use styles::{AppStyle, create_styles};

pub fn connection_view<'a>(ip: &str, error: Option<&str>, style: &AppStyle) -> Element<'a, Message> {
    let title = text("GPU Monitor").size(48).style(style.title_color);
    let subtitle = text("Enter collector address to connect").size(20).style(style.text_color);

    let ip_input = text_input("e.g. 192.168.1.100:9876 or 192.168.1.100", ip)
        .on_input(Message::IpChanged)
        .padding(16).size(20).width(Length::Fixed(450.0));

    let connect_addr = if ip.trim().is_empty() {
        "0.0.0.0:9876".to_string()
    } else {
        let t = ip.trim();
        if t.contains(':') { t.to_string() } else { format!("{}:9876", t) }
    };

    let connect_btn = button(text("Connect").size(20).style(style.value_color))
        .padding([12, 32]).style(style.button_theme()).on_press(Message::Connect(connect_addr));

    let error_text = error.map(|e| text(e).size(16).style(Color::from_rgb(1.0, 0.3, 0.3)));

    let mut content = column![title, vertical_space().height(8), subtitle,
        vertical_space().height(32), ip_input, vertical_space().height(16), connect_btn]
        .align_items(Alignment::Center).spacing(8);

    if let Some(e) = error_text {
        content = content.push(vertical_space().height(16)).push(e);
    }

    container(content).width(Length::Fill).height(Length::Fill)
        .center_x().center_y().style(style.container_theme()).into()
}

pub fn waiting_view<'a>(addr: &str, style: &AppStyle) -> Element<'a, Message> {
    let content = column![
        text("Connecting...").size(36).style(style.title_color),
        vertical_space().height(16),
        text(format!("Waiting for data from {}", addr)).size(20).style(style.text_color),
        vertical_space().height(32),
        button(text("Cancel").size(20).style(style.value_color))
            .padding([12, 32]).style(style.danger_button_theme()).on_press(Message::Disconnect),
    ].align_items(Alignment::Center);

    container(content).width(Length::Fill).height(Length::Fill)
        .center_x().center_y().style(style.container_theme()).into()
}

pub fn monitor_view<'a>(
    data: &SystemData,
    history: &[SystemData],
    collector_addr: &str,
    style: &AppStyle,
) -> Element<'a, Message> {
    let header = create_header(collector_addr, style);

    let gauges = dashboard::gauge_row(data, style);

    let left_detail = dashboard::cpu_detail_panel(data, style);
    let right_detail = if let Some(gpu) = &data.gpu {
        dashboard::gpu_detail_panel(gpu, style)
    } else {
        container(text("No GPU detected").size(14).style(style.text_color))
            .padding(20).width(Length::Fill).style(style.card_theme()).into()
    };

    let detail_row = row![left_detail, horizontal_space().width(16), right_detail]
        .width(Length::Fill);

    let charts = dashboard::history_chart(history, style);

    let content = column![
        header,
        vertical_space().height(16),
        gauges,
        vertical_space().height(14),
        detail_row,
        vertical_space().height(14),
        charts,
    ].padding(16);

    container(scrollable(content))
        .width(Length::Fill).height(Length::Fill)
        .style(style.container_theme()).into()
}

pub fn error_view<'a>(error: &str, collector_addr: &str, style: &AppStyle) -> Element<'a, Message> {
    let content = column![
        text("Connection Lost").size(36).style(Color::from_rgb(1.0, 0.3, 0.3)),
        vertical_space().height(16),
        text(error).size(20).style(style.text_color),
        vertical_space().height(8),
        text(format!("Collector: {}", collector_addr)).size(16).style(style.text_color),
        vertical_space().height(32),
        row![
            button(text("Reconnect").size(20).style(style.value_color))
                .padding([12, 32]).style(style.button_theme())
                .on_press(Message::Connect(collector_addr.to_string())),
            horizontal_space().width(16),
            button(text("Change Address").size(16).style(style.value_color))
                .padding([8, 24]).style(style.danger_button_theme())
                .on_press(Message::Disconnect),
        ].align_items(Alignment::Center),
    ].align_items(Alignment::Center);

    container(content).width(Length::Fill).height(Length::Fill)
        .center_x().center_y().style(style.container_theme()).into()
}

fn create_header<'a>(collector_addr: &str, style: &AppStyle) -> Element<'a, Message> {
    let icon_title = text("\u{25CF} Monitor").size(17).style(style.title_color);
    let addr_label = text(collector_addr).size(11).style(style.text_color);

    let style_btn = button(text("\u{25C9}").size(14).style(style.value_color))
        .padding([3, 8]).style(style.button_theme()).on_press(Message::ToggleStyle);

    let disconnect_btn = button(text("\u{2716}").size(12).style(style.value_color))
        .padding([3, 8]).style(style.danger_button_theme()).on_press(Message::Disconnect);

    container(
        row![icon_title, text("  ").size(1), addr_label, horizontal_space(), style_btn,
            horizontal_space().width(4), disconnect_btn]
            .align_items(Alignment::Center)
    ).padding([6, 14]).width(Length::Fill).style(style.header_theme()).into()
}