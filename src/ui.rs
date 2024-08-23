use std::rc::Rc;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, Paragraph},
    Frame,
};

use crate::app::{App, SelectedPanel};

pub fn ui(frame: &mut Frame, app: &App) {
    let (main_layout, main_page_layout, main_task_page_layout) = setup_layout(&frame);

    let selected_panel_style = Style::default().bg(Color::LightBlue);
    let mut task_list_panel_style = Style::default();
    let mut task_title_panel_style = Style::default();
    let mut task_main_panel_style = Style::default();
    match *app.get_selected_panel() {
        SelectedPanel::List(_) => task_list_panel_style = selected_panel_style,
        SelectedPanel::Title(_) => task_title_panel_style = selected_panel_style,
        SelectedPanel::Task(_) => task_main_panel_style = selected_panel_style,
        SelectedPanel::None => {}
    }

    render_task_list(frame, app, &main_page_layout[0], task_list_panel_style);
    render_task_title(frame, app, &main_task_page_layout[0], task_title_panel_style);
    render_task_main(frame, app, &main_task_page_layout[1], task_main_panel_style);

    let commands_list = Paragraph::new("<c c> - Exit \t <c n> - New task \t <c d> - Delete task")
        .block(Block::new().borders(Borders::ALL));
    frame.render_widget(commands_list, main_layout[1]);
}

fn setup_layout(frame: &Frame) -> (Rc<[Rect]>, Rc<[Rect]>, Rc<[Rect]>) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(frame.area());

    let main_page_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(main_layout[0]);

    let main_task_page_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(15), Constraint::Percentage(85)])
        .split(main_page_layout[1]);

    (main_layout, main_page_layout, main_task_page_layout)
}

fn render_task_list(frame: &mut Frame, app: &App, area: &Rect, style: Style) {
    let task_list_outer = Block::default().borders(Borders::ALL);

    let tasks_list = List::new(app.get_tasks_titles())
        .block(Block::new().borders(Borders::NONE))
        .style(style);
    let task_list_inner_area =
        task_list_outer.inner(add_margins(&area, [2, 0, 2, 0]));

    frame.render_widget(task_list_outer, *area);
    frame.render_widget(tasks_list, task_list_inner_area);
}

fn render_task_title(frame: &mut Frame, app: &App, area: &Rect, style: Style) {
    let task_title = Paragraph::new("This is a title")
        .block(Block::new().borders(Borders::ALL))
        .style(style);
    frame.render_widget(task_title, *area);
}

fn render_task_main(frame: &mut Frame, app: &App, area: &Rect, style: Style) {
    let task_main = Paragraph::new("Here the details of the task")
        .block(Block::new().borders(Borders::ALL))
        .style(style);
    frame.render_widget(task_main, *area);
}

fn add_margins(rect: &Rect, margins: [u16; 4]) -> Rect {
    Rect::new(
        rect.x.saturating_add(margins[0]),
        rect.y.saturating_add(margins[1]),
        rect.width.saturating_sub(margins[0] + margins[2]),
        rect.height.saturating_sub(margins[1] + margins[3]),
    )
}
