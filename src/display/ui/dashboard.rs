use iced::{
    widget::canvas::{self, Cache, Canvas, Geometry, Path, Stroke},
    widget::{column, container, row, text, vertical_space, horizontal_space, progress_bar},
    Color, Element, Length, Point, Rectangle, Size, Theme, Background,
};
use crate::shared::{SystemData, GpuData};
use crate::Message;
use super::AppStyle;

// ════════════════════════════════════════════════════════════════
//  THEME DETECTION
// ════════════════════════════════════════════════════════════════

fn is_light(style: &AppStyle) -> bool {
    let c = style.bg_color;
    (c.r + c.g + c.b) / 3.0 > 0.5
}

// ════════════════════════════════════════════════════════════════
//  ARC GAUGE
// ════════════════════════════════════════════════════════════════

struct ArcGauge {
    pct: f32,
    color: Color,
    label: String,
    detail: String,
    text_primary: Color,
    text_secondary: Color,
    bg_arc: Color,
    cache: Cache,
}

impl ArcGauge {
    fn new(pct: f32, color: Color, label: &str, detail: &str, style: &AppStyle) -> Self {
        Self {
            pct: pct.clamp(0.0, 100.0),
            color,
            label: label.into(),
            detail: detail.into(),
            text_primary: style.value_color,
            text_secondary: style.text_color,
            bg_arc: if is_light(style) {
                Color::from_rgba(0.75, 0.75, 0.78, 0.4)
            } else {
                Color::from_rgba(0.16, 0.16, 0.2, 0.6)
            },
            cache: Cache::new(),
        }
    }
}

impl canvas::Program<Message> for ArcGauge {
    type State = ();
    fn draw(&self, _: &(), r: &iced::Renderer, _: &Theme, b: Rectangle, _: iced::mouse::Cursor) -> Vec<Geometry> {
        let geo = self.cache.draw(r, b.size(), |f| {
            let (w, h) = (b.width, b.height);
            // Draw centered square inside available bounds
            let side = w.min(h);
            let cx = w / 2.0;
            let cy = h / 2.0 - side * 0.06;
            let rad = side * 0.36;
            let sw = (rad * 0.14).max(5.0);
            let (s, e) = (135.0_f32.to_radians(), 405.0_f32.to_radians());
            let sweep = e - s;

            f.stroke(&arc_path(cx, cy, rad, s, e, 64),
                Stroke::default().with_color(self.bg_arc).with_width(sw).with_line_cap(canvas::LineCap::Round));

            let p = self.pct / 100.0;
            if p > 0.002 {
                f.stroke(&arc_path(cx, cy, rad, s, s + sweep * p, 32.max((p * 64.0) as u32)),
                    Stroke::default().with_color(self.color).with_width(sw).with_line_cap(canvas::LineCap::Round));
            }

            let pct_size = (side * 0.14).max(12.0);
            let detail_size = (side * 0.065).max(8.0);
            let label_size = (side * 0.075).max(9.0);

            f.fill_text(canvas::Text {
                content: format!("{:.1}%", self.pct),
                position: Point::new(cx, cy - side * 0.02),
                color: self.text_primary,
                size: iced::Pixels(pct_size),
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
                ..Default::default()
            });

            f.fill_text(canvas::Text {
                content: self.detail.clone(),
                position: Point::new(cx, cy + rad * 0.25),
                color: self.text_secondary,
                size: iced::Pixels(detail_size),
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
                ..Default::default()
            });

            f.fill_text(canvas::Text {
                content: self.label.clone(),
                position: Point::new(cx, h - side * 0.05),
                color: self.text_secondary,
                size: iced::Pixels(label_size),
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
                ..Default::default()
            });
        });
        vec![geo]
    }
}

fn arc_path(cx: f32, cy: f32, r: f32, s: f32, e: f32, n: u32) -> Path {
    Path::new(|b| {
        for i in 0..=n {
            let t = i as f32 / n as f32;
            let a = s + (e - s) * t;
            let p = Point::new(cx + r * a.cos(), cy + r * a.sin());
            if i == 0 { b.move_to(p); } else { b.line_to(p); }
        }
    })
}

/// Single gauge card: equal Fill width, generous Fixed height.
/// The canvas inside draws a centered square using min(w, h).
fn gauge<'a>(label: &str, detail: &str, pct: f32, color: Color, style: &AppStyle) -> Element<'a, Message> {
    container(
        Canvas::new(ArcGauge::new(pct, color, label, detail, style))
            .width(Length::Fill)
            .height(Length::Fill)
    )
    .width(Length::Fill)
    .height(Length::Fixed(200.0))
    .center_x()
    .center_y()
    .padding(4)
    .style(style.card_theme())
    .into()
}

// ════════════════════════════════════════════════════════════════
//  GAUGE ROW
// ════════════════════════════════════════════════════════════════

pub fn gauge_row<'a>(data: &SystemData, style: &AppStyle) -> Element<'a, Message> {
    let cpu = gauge("CPU", &format!("{:.0} MHz", data.cpu_frequency), data.cpu_usage, style.cpu_color, style);
    let mem = gauge("Memory", &format!("{}/{} MB", data.memory_used, data.memory_total), data.memory_usage, style.memory_color, style);

    if let Some(gpu) = &data.gpu {
        let g = gauge("GPU", &format!("{:.1}%", gpu.utilization), gpu.utilization, style.gpu_color, style);
        let v = gauge("VRAM", &format!("{}/{} MB", gpu.memory_used, gpu.memory_total), gpu.memory_utilization, style.gpu_memory_color, style);
        row![cpu, mem, g, v].spacing(12).width(Length::Fill).into()
    } else {
        row![cpu, mem].spacing(12).width(Length::Fill).into()
    }
}

// ════════════════════════════════════════════════════════════════
//  CPU DETAIL PANEL
// ════════════════════════════════════════════════════════════════

pub fn cpu_detail_panel<'a>(data: &SystemData, style: &AppStyle) -> Element<'a, Message> {
    let title = text("CPU Details").size(16).style(style.title_color);

    let freq_val = format!("{:.0} MHz", data.cpu_frequency);
    let temp_val = if data.cpu_temperature > 0.0 {
        format!("{:.0}\u{00b0}C", data.cpu_temperature)
    } else {
        "N/A".to_string()
    };

    let temp_color = if data.cpu_temperature > 85.0 {
        Color::from_rgb(1.0, 0.3, 0.3)
    } else if data.cpu_temperature > 65.0 {
        Color::from_rgb(1.0, 0.8, 0.0)
    } else if data.cpu_temperature > 0.0 {
        Color::from_rgb(0.3, 1.0, 0.3)
    } else {
        style.text_color
    };

    let temp_pct = if data.cpu_temperature > 0.0 { data.cpu_temperature.min(100.0) } else { 0.0 };

    let freq_bar = bar_item("Frequency", &freq_val, (data.cpu_frequency / 8000.0 * 100.0).min(100.0), Color::from_rgb(0.3, 0.8, 1.0), style);
    let temp_bar = bar_item("Temperature", &temp_val, temp_pct, temp_color, style);

    container(
        column![
            title,
            vertical_space().height(14),
            row![freq_bar, horizontal_space().width(16), temp_bar].width(Length::Fill)
        ]
    ).padding(18).width(Length::Fill).style(style.card_theme()).into()
}

// ════════════════════════════════════════════════════════════════
//  GPU DETAIL PANEL
// ════════════════════════════════════════════════════════════════

pub fn gpu_detail_panel<'a>(gpu: &GpuData, style: &AppStyle) -> Element<'a, Message> {
    let title = text(format!("GPU: {}", gpu.name.replace("NVIDIA ", "")))
        .size(16).style(style.title_color);

    let temp_color = if gpu.temperature > 80.0 {
        Color::from_rgb(1.0, 0.3, 0.3)
    } else if gpu.temperature > 60.0 {
        Color::from_rgb(1.0, 0.8, 0.0)
    } else {
        Color::from_rgb(0.3, 1.0, 0.3)
    };

    let t = bar_item("Temperature", &format!("{:.0}\u{00b0}C", gpu.temperature), gpu.temperature.min(100.0), temp_color, style);
    let p = bar_item("Power", &format!("{:.1}W", gpu.power_usage), (gpu.power_usage / 300.0 * 100.0).min(100.0), Color::from_rgb(1.0, 0.8, 0.2), style);
    let fan = bar_item("Fan", &format!("{:.0}%", gpu.fan_speed), gpu.fan_speed, Color::from_rgb(0.4, 0.8, 1.0), style);
    let clock = text(format!("Clock: {} MHz", gpu.clock_speed)).size(13).style(style.text_color);

    container(
        column![
            row![title, horizontal_space(), clock].align_items(iced::Alignment::Center),
            vertical_space().height(14),
            row![t, horizontal_space().width(12), p, horizontal_space().width(12), fan].width(Length::Fill)
        ]
    ).padding(18).width(Length::Fill).style(style.card_theme()).into()
}

// ════════════════════════════════════════════════════════════════
//  BAR ITEM
// ════════════════════════════════════════════════════════════════

fn bar_item<'a>(label: &str, value: &str, pct: f32, color: Color, style: &AppStyle) -> Element<'a, Message> {
    let bar_bg = if is_light(style) {
        Color::from_rgba(0.82, 0.82, 0.84, 0.7)
    } else {
        Color::from_rgba(0.14, 0.14, 0.18, 0.7)
    };

    let bar = progress_bar(0.0..=100.0, pct.clamp(0.0, 100.0))
        .height(7).width(Length::Fill)
        .style(move |_: &iced::Theme| iced::widget::progress_bar::Appearance {
            background: Background::Color(bar_bg),
            bar: Background::Color(color),
            border_radius: 4.0.into(),
        });

    container(column![
        row![text(label).size(12).style(style.text_color), horizontal_space(),
            text(value).size(14).style(style.value_color)]
            .align_items(iced::Alignment::Center),
        vertical_space().height(6), bar
    ]).width(Length::Fill).into()
}

// ════════════════════════════════════════════════════════════════
//  HISTORY CHARTS
// ════════════════════════════════════════════════════════════════

pub fn history_chart<'a>(history: &[SystemData], style: &AppStyle) -> Element<'a, Message> {
    if history.is_empty() {
        return container(column![
            text("History").size(16).style(style.title_color),
            vertical_space().height(12),
            text("Waiting for data...").size(14).style(style.text_color),
        ]).width(Length::Fill).height(Length::Fixed(180.0))
            .center_x().center_y().style(style.card_theme()).into();
    }

    let cv: Vec<f32> = history.iter().map(|d| d.cpu_usage).collect();
    let mv: Vec<f32> = history.iter().map(|d| d.memory_usage).collect();
    let gv: Vec<f32> = history.iter().filter_map(|d| d.gpu.as_ref().map(|g| g.utilization)).collect();
    let h = 150.0;

    let light = is_light(style);
    let chart_bg = if light { Color::from_rgba(0.92, 0.92, 0.94, 0.4) } else { Color::from_rgba(0.05, 0.05, 0.07, 0.4) };
    let grid_color = if light { Color::from_rgba(0.7, 0.7, 0.73, 0.5) } else { Color::from_rgba(0.16, 0.16, 0.2, 0.5) };
    let axis_color = if light { Color::from_rgba(0.35, 0.35, 0.4, 1.0) } else { Color::from_rgba(0.4, 0.4, 0.45, 1.0) };
    let placeholder_color = if light { Color::from_rgba(0.45, 0.45, 0.5, 1.0) } else { Color::from_rgba(0.4, 0.4, 0.45, 1.0) };

    let cs = column![
        text("CPU History").size(12).style(style.cpu_color), vertical_space().height(3),
        Canvas::new(LineChart { values: cv, color: style.cpu_color, chart_bg, grid_color, axis_color, placeholder_color })
            .width(Length::Fill).height(Length::Fixed(h)),
    ];
    let ms = column![
        text("Memory History").size(12).style(style.memory_color), vertical_space().height(3),
        Canvas::new(LineChart { values: mv, color: style.memory_color, chart_bg, grid_color, axis_color, placeholder_color })
            .width(Length::Fill).height(Length::Fixed(h)),
    ];

    let mut c = column![
        text("History").size(16).style(style.title_color), vertical_space().height(10),
        row![cs, horizontal_space().width(14), ms].width(Length::Fill),
    ];

    if !gv.is_empty() {
        c = c.push(vertical_space().height(12));
        c = c.push(column![
            text("GPU History").size(12).style(style.gpu_color), vertical_space().height(3),
            Canvas::new(LineChart { values: gv, color: style.gpu_color, chart_bg, grid_color, axis_color, placeholder_color })
                .width(Length::Fill).height(Length::Fixed(h)),
        ]);
    }

    container(c).padding(18).width(Length::Fill).style(style.card_theme()).into()
}

// ════════════════════════════════════════════════════════════════
//  LINE CHART
// ════════════════════════════════════════════════════════════════

struct LineChart {
    values: Vec<f32>,
    color: Color,
    chart_bg: Color,
    grid_color: Color,
    axis_color: Color,
    placeholder_color: Color,
}

impl canvas::Program<Message> for LineChart {
    type State = ();
    fn draw(&self, _: &(), r: &iced::Renderer, _: &Theme, b: Rectangle, _: iced::mouse::Cursor) -> Vec<Geometry> {
        let geo = Cache::new().draw(r, b.size(), |f| {
            let (w, h) = (b.width, b.height);
            let pad = 36.0;
            f.fill_rectangle(Point::ORIGIN, Size::new(w, h), self.chart_bg);

            if self.values.len() < 2 {
                f.fill_text(canvas::Text {
                    content: "Collecting...".into(),
                    position: Point::new(w / 2.0, h / 2.0),
                    color: self.placeholder_color,
                    size: iced::Pixels(12.0),
                    horizontal_alignment: iced::alignment::Horizontal::Center,
                    vertical_alignment: iced::alignment::Vertical::Center,
                    ..Default::default()
                });
                return;
            }

            let (l, t, cw, ch) = (pad, 8.0, w - pad - 8.0, h - 26.0);
            let max = self.values.iter().copied().fold(0.0f32, f32::max).max(100.0);
            let step = if self.values.len() > 1 { cw / (self.values.len() as f32 - 1.0) } else { cw };

            for i in 0..=4 {
                let y = t + (i as f32 / 4.0) * ch;
                f.stroke(&Path::line(Point::new(l, y), Point::new(l + cw, y)),
                    Stroke::default().with_color(self.grid_color).with_width(0.5));
                f.fill_text(canvas::Text {
                    content: format!("{:.0}", max * (4 - i) as f32 / 4.0),
                    position: Point::new(l - 5.0, y),
                    color: self.axis_color,
                    size: iced::Pixels(9.0),
                    horizontal_alignment: iced::alignment::Horizontal::Right,
                    vertical_alignment: iced::alignment::Vertical::Center,
                    ..Default::default()
                });
            }

            let pts: Vec<Point> = self.values.iter().enumerate().map(|(i, &v)|
                Point::new(l + i as f32 * step, t + ch - (v / max) * ch)
            ).collect();

            for i in 0..pts.len() - 1 {
                let (p1, p2) = (pts[i], pts[i + 1]);
                let area = Path::new(|b| {
                    b.move_to(Point::new(p1.x, t + ch));
                    b.line_to(p1);
                    b.line_to(p2);
                    b.line_to(Point::new(p2.x, t + ch));
                    b.close();
                });
                f.fill(&area, Color::from_rgba(self.color.r, self.color.g, self.color.b, 0.05));
            }
            for i in 0..pts.len() - 1 {
                f.stroke(&Path::line(pts[i], pts[i + 1]),
                    Stroke::default().with_color(self.color).with_width(1.8));
            }
            for p in &pts {
                f.fill(&Path::circle(*p, 1.8), self.color);
            }

            if let (Some(last), Some(val)) = (pts.last(), self.values.last()) {
                f.fill_text(canvas::Text {
                    content: format!("{:.1}%", val),
                    position: Point::new(last.x, last.y - 10.0),
                    color: self.color,
                    size: iced::Pixels(10.0),
                    horizontal_alignment: iced::alignment::Horizontal::Center,
                    vertical_alignment: iced::alignment::Vertical::Center,
                    ..Default::default()
                });
            }
        });
        vec![geo]
    }
}
