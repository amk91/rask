use chrono::Local;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{
    task::{Task, TaskStatus},
    App, SelectedPanel, SelectedTaskField,
};

pub fn ui(frame: &mut Frame, app: &App) {
    let (task_list_area, task_title_area, task_main_area) = setup_layout(&frame);

    let mut selected_index = None;
    let mut selected_task_field = None;
    match app.get_selected_panel() {
        SelectedPanel::List(value) => {
            selected_index = value;
        }
        SelectedPanel::Title(value) => {
            selected_index = value;
        }
        SelectedPanel::Task(value) => {
            if let Some((index, task_field)) = value {
                selected_index = Some(index);
                selected_task_field = Some(task_field);
            }
        }
        SelectedPanel::None => {}
    }

    let task = if let Some(index) = selected_index {
        app.get_task_by_index(index)
    } else {
        None
    };

    render_task_list(frame, app, task_list_area, selected_index);
    render_task_title(frame, app, task_title_area);
    render_task_main(frame, task, task_main_area, selected_task_field);
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

fn render_task_list(frame: &mut Frame, app: &App, area: Rect, index: Option<usize>) {
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

    let tasks_list = List::new(items);
    let tasks_list_area = get_rendered_widget_block_area(
        frame,
        area,
        Borders::ALL,
        BorderType::QuadrantOutside,
        Style::default(),
        [1, 0, 1, 0],
    );

    frame.render_widget(tasks_list, tasks_list_area);
}

fn render_task_title(frame: &mut Frame, app: &App, area: Rect) {
    let task_title_text = match app.get_selected_panel() {
        SelectedPanel::List(Some(index))
        | SelectedPanel::Title(Some(index))
        | SelectedPanel::Task(Some((index, _))) => {
            if let Some(task) = app.get_task_by_index(index) {
                Text::from(task.title.clone())
            } else {
                Text::raw("")
            }
        }
        _ => Text::raw(""),
    };

    let task_title = Paragraph::new(task_title_text);
    let task_title_area = get_rendered_widget_block_area(
        frame,
        area,
        Borders::ALL,
        BorderType::Plain,
        Style::default(),
        [1, 0, 1, 0],
    );

    frame.render_widget(task_title, task_title_area);
}

fn render_task_main(
    frame: &mut Frame,
    task: Option<&Task>,
    area: Rect,
    selected_task_field: Option<SelectedTaskField>,
) {
    let task_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(4),
            Constraint::Length(3),
            Constraint::Min(6),
        ])
        .split(area);
    let (status_area, complete_by_area, description_area) =
        (task_areas[0], task_areas[1], task_areas[2]);

    frame.render_widget(Block::default().borders(Borders::ALL), area);

    render_task_status(
        frame,
        task,
        add_margins(&status_area, [1, 0, 1, 0]),
        selected_task_field,
    );
    render_task_complete_by(
        frame,
        task,
        add_margins(&complete_by_area, [1, 0, 1, 0]),
        selected_task_field,
    );
    render_task_description(
        frame,
        task,
        add_margins(&description_area, [1, 0, 1, 0]),
        selected_task_field,
    );
}

fn render_task_status(
    frame: &mut Frame,
    task: Option<&Task>,
    area: Rect,
    selected_task_field: Option<SelectedTaskField>,
) {
    let task_status_text = Text::from(if let Some(task) = task {
        match task.status {
            TaskStatus::New => "New",
            TaskStatus::InProgress => "In progress",
            TaskStatus::Completed => "Completed",
        }
    } else {
        ""
    });

    let task_status = Paragraph::new(task_status_text);
    let task_status_area = get_rendered_widget_block_area(
        frame,
        area,
        Borders::BOTTOM,
        BorderType::Plain,
        Style::default().bg(
            if let Some(SelectedTaskField::Status) = selected_task_field {
                Color::DarkGray
            } else {
                Color::Reset
            },
        ),
        [1, 2, 1, 0],
    );

    frame.render_widget(task_status, task_status_area);
}

fn render_task_complete_by(
    frame: &mut Frame,
    task: Option<&Task>,
    area: Rect,
    selected_task_field: Option<SelectedTaskField>,
) {
    let task_complete_by_text = Text::from(
        task.and_then(|t| {
            t.complete_by
                .and_then(|d| Some(d.with_timezone(&Local).format("%Y-%m-%d %H:%M").to_string()))
        })
        .unwrap_or_default(),
    );

    let task_complete_by = Paragraph::new(task_complete_by_text);
    let task_complete_by_area = get_rendered_widget_block_area(
        frame,
        area,
        Borders::BOTTOM,
        BorderType::Plain,
        Style::default().bg(
            if let Some(SelectedTaskField::CompleteBy) = selected_task_field {
                Color::DarkGray
            } else {
                Color::Reset
            },
        ),
        [1, 1, 1, 0],
    );

    frame.render_widget(task_complete_by, task_complete_by_area);
}

fn render_task_description(
    frame: &mut Frame,
    task: Option<&Task>,
    area: Rect,
    selected_task_field: Option<SelectedTaskField>,
) {
    let task_description_text =
        Text::from(task.and_then(|t| t.description.clone()).unwrap_or_default());

    let task_description = Paragraph::new(task_description_text).wrap(Wrap { trim: true });
    let task_description_area = get_rendered_widget_block_area(
        frame,
        area,
        Borders::NONE,
        BorderType::Plain,
        Style::default().bg(
            if let Some(SelectedTaskField::Description) = selected_task_field {
                Color::DarkGray
            } else {
                Color::Reset
            },
        ),
        [1, 1, 1, 1],
    );

    frame.render_widget(task_description, task_description_area);
}

fn add_margins(rect: &Rect, margins: [u16; 4]) -> Rect {
    Rect::new(
        rect.x.saturating_add(margins[0]),
        rect.y.saturating_add(margins[1]),
        rect.width.saturating_sub(margins[0] + margins[2]),
        rect.height.saturating_sub(margins[1] + margins[3]),
    )
}

fn get_rendered_widget_block_area(
    frame: &mut Frame,
    area: Rect,
    borders: Borders,
    border_type: BorderType,
    style: Style,
    margins: [u16; 4],
) -> Rect {
    let widget_block = Block::new()
        .border_type(border_type)
        .borders(borders)
        .style(style);
    let widget_area = widget_block.inner(add_margins(&area, margins));

    frame.render_widget(widget_block, area);

    widget_area
}
