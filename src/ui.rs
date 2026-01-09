use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::Marker,
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Circle},
        Block, Borders, Paragraph,
    },
    Frame,
};

use crate::app::App;

/// Main render function
pub fn render(frame: &mut Frame, app: &App) {
    let main_layout = Layout::vertical([
        Constraint::Min(35),   // DIP switches section
        Constraint::Length(8), // Configuration section
        Constraint::Length(3), // Keybindings
    ]);

    let [switches_area, config_area, keys_area] = main_layout.areas(frame.area());

    render_switches(frame, switches_area, app);
    render_config(frame, config_area, app);
    render_keybindings(frame, keys_area);
}

/// Render all 8 DIP switches
fn render_switches(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("GEN10 DIP Switch Simulator")
        .title_style(Style::default().fg(Color::Cyan).bold());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Switch labels configuration
    let switch_configs: [(Option<&str>, &str, Option<&str>); 8] = [
        (Some("SD"), "OUT 1-4", Some("HD")),
        (Some("SD"), "OUT 5-6", Some("HD")),
        (Some("BLK"), "SD OUT", Some("BARS")),
        (Some("59.94"), "FORMAT", Some("50")),
        (Some("1080"), "HD FMT", Some("720")),
        (None, "S1", None),
        (None, "S2", None),
        (None, "S3", None),
    ];

    // Calculate spacing for 8 switches
    let switch_height = 4;
    let available_height = inner.height as usize;
    let total_switch_height = switch_height * 8;
    let spacing = if available_height > total_switch_height {
        (available_height - total_switch_height) / 9
    } else {
        0
    };

    let mut y_offset = inner.y + spacing as u16;

    for (i, config) in switch_configs.iter().enumerate() {
        let switch_area = Rect::new(inner.x, y_offset, inner.width, switch_height as u16);
        render_single_switch(frame, switch_area, i, app, config);
        y_offset += switch_height as u16 + spacing as u16;
    }
}

/// Render a single DIP switch with labels and canvas
fn render_single_switch(
    frame: &mut Frame,
    area: Rect,
    index: usize,
    app: &App,
    labels: &(Option<&str>, &str, Option<&str>),
) {
    let (left_label, center_label, right_label) = labels;
    let position = app.animations[index].position();
    let state = app.dipswitch.get(index);

    // Layout: [left_label][number][canvas][right_label]
    let horizontal = Layout::horizontal([
        Constraint::Length(10), // Left label
        Constraint::Length(3),  // Switch number
        Constraint::Min(20),    // Canvas (flexible)
        Constraint::Length(10), // Right label
    ]);

    let parts = horizontal.split(area);

    // Render left label
    if let Some(label) = left_label {
        let label_color = if !state { Color::White } else { Color::DarkGray };
        frame.render_widget(
            Paragraph::new(*label)
                .style(Style::default().fg(label_color))
                .right_aligned(),
            parts[0],
        );
    }

    // Render switch number
    frame.render_widget(
        Paragraph::new(format!("{}", index + 1))
            .style(Style::default().fg(Color::White).bold())
            .centered(),
        parts[1],
    );

    // Render canvas switch
    let canvas_area = parts[2];
    let switch_canvas = create_switch_canvas(position, state);
    frame.render_widget(switch_canvas, canvas_area);

    // Render center label below canvas
    let label_area = Rect::new(
        canvas_area.x,
        canvas_area.y + 2,
        canvas_area.width,
        1,
    );
    frame.render_widget(
        Paragraph::new(*center_label)
            .style(Style::default().fg(Color::Gray))
            .centered(),
        label_area,
    );

    // Render right label
    if let Some(label) = right_label {
        let label_color = if state { Color::Green } else { Color::DarkGray };
        frame.render_widget(
            Paragraph::new(*label)
                .style(Style::default().fg(label_color))
                .left_aligned(),
            parts[3],
        );
    }
}

/// Create a Canvas widget for a single switch
fn create_switch_canvas(position: f64, _state: bool) -> Canvas<'static, impl Fn(&mut ratatui::widgets::canvas::Context)> {
    Canvas::default()
        .marker(Marker::Braille)
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 50.0])
        .paint(move |ctx| {
            // Calculate circle position based on animation
            let circle_x = 10.0 + (position * 80.0);

            // Calculate color gradient from gray to green
            let fill_amount = position;
            let red = (128.0 - fill_amount * 128.0) as u8;
            let green = (128.0 + fill_amount * 127.0) as u8;
            let blue = (128.0 - fill_amount * 128.0) as u8;
            let color = Color::Rgb(red, green, blue);

            // Draw filled circle
            ctx.draw(&Circle {
                x: circle_x,
                y: 25.0,
                radius: 8.0,
                color,
            });

            // Draw inner fill circles for solid appearance
            if fill_amount > 0.01 {
                for r in (0..=(6.0 * fill_amount) as u32).step_by(1) {
                    ctx.draw(&Circle {
                        x: circle_x,
                        y: 25.0,
                        radius: r as f64,
                        color: Color::Green,
                    });
                }
            }
        })
}

/// Render configuration display section
fn render_config(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("CURRENT CONFIGURATION")
        .title_style(Style::default().fg(Color::Cyan).bold());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let config = app.dipswitch.get_config();

    // Determine colors based on format type
    let group1_color = if config.group1_format.starts_with("HD") {
        Color::Green
    } else {
        Color::Yellow
    };

    let group2_color = if config.group2_format.starts_with("HD") {
        Color::Green
    } else {
        Color::Yellow
    };

    let lines = vec![
        Line::from(vec![
            Span::raw("  Group 1 (Out 1-4):  "),
            Span::styled(&config.group1_format, Style::default().fg(group1_color)),
        ]),
        Line::from(vec![
            Span::raw("  Group 2 (Out 5-6):  "),
            Span::styled(&config.group2_format, Style::default().fg(group2_color)),
        ]),
        Line::from(vec![
            Span::raw("  SD Output:          "),
            Span::styled(config.sd_video.as_str(), Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::raw("  Audio (AES-11):     "),
            Span::styled(config.audio.as_str(), Style::default().fg(Color::White)),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::raw("  Video Format Bitmask: "),
            Span::styled(
                app.dipswitch.get_bitmask_binary(),
                Style::default().fg(Color::White),
            ),
            Span::raw("  "),
            Span::styled(
                app.dipswitch.get_bitmask_hex(),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::styled("                        87654321", Style::default().fg(Color::DarkGray)),
    ];

    frame.render_widget(Paragraph::new(lines), inner);
}

/// Render keybindings help bar
fn render_keybindings(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("KEYBINDINGS")
        .title_style(Style::default().fg(Color::Cyan).bold());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let text = Line::from(vec![
        Span::styled("[1-8]", Style::default().fg(Color::Yellow).bold()),
        Span::raw(" Toggle Switch  |  "),
        Span::styled("[R]", Style::default().fg(Color::Yellow).bold()),
        Span::raw(" Reset  |  "),
        Span::styled("[Q]", Style::default().fg(Color::Yellow).bold()),
        Span::raw(" Quit"),
    ]);

    frame.render_widget(Paragraph::new(text).centered(), inner);
}
