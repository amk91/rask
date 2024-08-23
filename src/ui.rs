use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{App, SelectedPanel};

pub fn ui(frame: &mut Frame, app: &App) {
    let (task_list_area, task_title_area, task_main_area) = setup_layout(&frame);

    let selected_panel_style = Style::default().bg(Color::DarkGray);
    let mut selected_index = None;
    let mut task_title_panel_style = Style::default();
    let mut task_main_panel_style = Style::default();
    match *app.get_selected_panel() {
        SelectedPanel::List(index) => {
            selected_index = index;
        }
        SelectedPanel::Title(_) => task_title_panel_style = selected_panel_style,
        SelectedPanel::Task(_) => task_main_panel_style = selected_panel_style,
        SelectedPanel::None => {}
    }

    render_task_list(frame, app, task_list_area, Style::default(), selected_index);
    render_task_title(frame, app, task_title_area, task_title_panel_style);
    render_task_main(frame, app, task_main_area, task_main_panel_style);
}

fn setup_layout(frame: &Frame) -> (Rect, Rect, Rect) {
    let main_page_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(frame.area());

    let main_task_page_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(15), Constraint::Percentage(85)])
        .split(main_page_layout[1]);

    (
        main_page_layout[0],
        main_task_page_layout[0],
        main_task_page_layout[1],
    )
}

fn render_task_list(frame: &mut Frame, app: &App, area: Rect, style: Style, index: Option<usize>) {
    let listitem_default_style = Style::default();
    let listitem_selected_style = Style::default().bg(Color::LightBlue);

    let mut items: Vec<ListItem> = app
        .get_tasks_titles()
        .iter()
        .map(|title| ListItem::new(title.clone()).style(listitem_default_style))
        .collect();
    if let Some(index) = index {
        items[index] = items[index].clone().style(listitem_selected_style);
    }

    let tasks_list = List::new(items)
        .block(Block::new().borders(Borders::NONE))
        .style(style);
    let task_list_outer = Block::default().borders(Borders::ALL);
    let task_list_inner_area = task_list_outer.inner(add_margins(&area, [0, 0, 0, 0]));

    frame.render_widget(task_list_outer, area);
    frame.render_widget(tasks_list, task_list_inner_area);
}

fn render_task_title(frame: &mut Frame, app: &App, area: Rect, style: Style) {
    let task_title = Paragraph::new("This is a title")
        .block(Block::new().borders(Borders::ALL))
        .style(style);
    frame.render_widget(task_title, area);
}

fn render_task_main(frame: &mut Frame, app: &App, area: Rect, style: Style) {
    let task_main = Paragraph::new("Here the details of the task")
        .block(Block::new().borders(Borders::ALL))
        .style(style);
    frame.render_widget(task_main, area);
}

fn add_margins(rect: &Rect, margins: [u16; 4]) -> Rect {
    Rect::new(
        rect.x.saturating_add(margins[0]),
        rect.y.saturating_add(margins[1]),
        rect.width.saturating_sub(margins[0] + margins[2]),
        rect.height.saturating_sub(margins[1] + margins[3]),
    )
}
