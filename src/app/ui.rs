use symbols::line;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Cell, LineGauge, Row, Table, Tabs};
use tui::{symbols, Frame};
use tui_logger::TuiLoggerWidget;
use super::actions::Actions;
use crate::app::App;
use crate::disp_mgr::disp::DispProp;

///Main draw function.
pub fn draw<B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let size = rect.size();
    check_size(&size);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(7),
                Constraint::Percentage(93),
            ]
            .as_ref(),
        )
        .split(size);

    if let (Some(dm), Some(tab_index)) = (app.state().disp_mgr(), app.state().tab_index()) {
        rect.render_widget(draw_tabs(&tab_index, dm.get_name_list()), chunks[0]);
        if tab_index == dm.get_num_disps() {
            draw_menu_debug(rect, app, chunks);
        } else {
            draw_menu_controller(rect, app, chunks, &tab_index);
        }
    }
}

///Draw "tabs" at top of screen
fn draw_tabs<'a>(index: &usize, mut names: Vec<&'a str>) -> Tabs<'a> {
    names.push("Debug");
    let titles = names
        .iter()
        .map(|t| {
            Spans::from(vec![
                Span::styled(*t, Style::default().fg(Color::White)),
            ])
        })
        .collect();

    Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("tuib").title_alignment(Alignment::Center))
        .select(*index)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
            .fg(Color::Blue))
}

///Draw the debug menu
pub fn draw_menu_debug<B>(rect: &mut Frame<B>, app: &App, chunks: Vec<Rect>)
where
    B: Backend,
{
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(80),
                Constraint::Percentage(20),
            ].as_ref(),
        )
        .split(chunks[1]);

    let logs = draw_logs();
    rect.render_widget(logs, body_chunks[0]);

    let help = draw_help(app.actions());
    rect.render_widget(help, body_chunks[1]);
}

///Draw the controller menu. Displays different stats depending which display is connected
pub fn draw_menu_controller<B>(rect: &mut Frame<B>, app: &App, chunks: Vec<Rect>, tab_index: &usize)
where
    B: Backend,
{
    let body_chunks = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin((rect.size().width / 5) as u16)
        .vertical_margin((rect.size().height / 3) as u16)
        .constraints([
                Constraint::Length((rect.size().height / 12) as u16),
                Constraint::Length((rect.size().height / 12) as u16),
                Constraint::Length((rect.size().height / 12) as u16),
                Constraint::Length((rect.size().height / 12) as u16),
            ].as_ref(),
        )
        .split(chunks[1]);

    if let (Some(dm), Some(focused_prop)) = (app.state().disp_mgr(), app.state().focused_prop()) {
        // let disp = &app.state().disp_mgr().disps[*tab_index].clone();
        let disp = dm.get_disp_by_index(*tab_index);
        let brightness = (disp.brightness as f64) / 100.0;
        let r = (disp.gamma.r as f64) / 100.0;
        let g = (disp.gamma.g as f64) / 100.0;
        let b = (disp.gamma.b as f64) / 100.0;

        let gauge = draw_gauge("Brightness".to_string(), brightness, Color::DarkGray, focused_prop, DispProp::Brightness);
        rect.render_widget(gauge, body_chunks[0]);
        let gauge = draw_gauge("Red".to_string(), r, Color::Red, focused_prop, DispProp::R);
        rect.render_widget(gauge, body_chunks[1]);
        let gauge = draw_gauge("Green".to_string(), g, Color::Green, focused_prop, DispProp::G);
        rect.render_widget(gauge, body_chunks[2]);
        let gauge = draw_gauge("Blue".to_string(), b, Color::Blue, focused_prop, DispProp::B);
        rect.render_widget(gauge, body_chunks[3]);
    }
}

///Draw a gauge meant to display brightness, r, g, or b status of a display. Drawn differently if selected.
fn draw_gauge(mut title: String, mut ratio: f64, color: Color, focused_prop: DispProp, gauge_prop: DispProp) -> LineGauge<'static> {
    ratio = ratio.clamp(0.0, 1.0);
    if focused_prop == gauge_prop {
        title.push(']');
        title.insert(0, '[');
        let span = Span::styled(title, Style::default().add_modifier(Modifier::BOLD));
        LineGauge::default()
            .block(Block::default().borders(Borders::NONE).title(span))
            .gauge_style(Style::default().fg(color).add_modifier(Modifier::BOLD))
            .line_set(line::THICK)
            .ratio(ratio)
    } else {
        LineGauge::default()
            .block(Block::default().borders(Borders::NONE).title(title))
            .gauge_style(Style::default().fg(color))
            .line_set(line::NORMAL)
            .ratio(ratio)
    }
}

//Draw menu that shows keybindings
fn draw_help(actions: &Actions) -> Table {
    let key_style = Style::default().fg(Color::LightCyan);
    let help_style = Style::default().fg(Color::Gray);

    let mut rows = vec![];
    for action in actions.actions().iter() {
        let mut first = true;
        for key in action.keys() {
            let help = if first {
                first = false;
                action.to_string()
            } else {
                String::from("")
            };
            let row = Row::new(vec![
                Cell::from(Span::styled(key.to_string(), key_style)),
                Cell::from(Span::styled(help, help_style)),
            ]);
            rows.push(row);
        }
    }

    Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title("Help"),
        )
        .widths(&[Constraint::Length(11), Constraint::Min(20)])
        .column_spacing(1)
}

///Draw tuiloggerwidget
fn draw_logs<'a>() -> TuiLoggerWidget<'a> {
    TuiLoggerWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Gray))
        .style_info(Style::default().fg(Color::Blue))
        .block(
            Block::default()
                .title("Logs")
                .border_style(Style::default().fg(Color::White).bg(Color::Black))
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black))
}

///Ensure window size is valid
fn check_size(rect: &Rect) {
    if rect.width < 52 {
        panic!("Require width >= 52, (got {})", rect.width);
    }
    if rect.height < 28 {
        panic!("Require height >= 28, (got {})", rect.height);
    }
}
