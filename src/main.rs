use ratatui::{
    crossterm::{
        cursor::EnableBlinking,
        event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
        execute,
        terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    prelude::CrosstermBackend,
    Frame, Terminal,
};
use std::{
    io::{self, prelude::*, Result},
    path::PathBuf,
};

// use rand::Rng;
// use uuid::Uuid;
// use chrono::offset;

mod ui;
use ui::ui;

mod app;
use app::{
    App, SelectedPanel, SelectedTaskField,
};

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableBlinking)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Piece of code to generate valid JSON for serde serialization
    // {
    //     let node: [u8; 6] = rand::thread_rng().gen();
    //     let now = offset::Utc::now();
    //     let tasks = vec![
    //         Task {
    //             uuid: Uuid::now_v6(&node),
    //             title: "Something".into(),
    //             created: now,
    //             complete_by: None,
    //             status: TaskStatus::New,
    //             percentage: Some(10),
    //         },
    //         Task {
    //             uuid: Uuid::now_v6(&node),
    //             title: "Another".into(),
    //             created: now,
    //             complete_by: None,
    //             status: TaskStatus::New,
    //             percentage: Some(50),
    //         },
    //         Task {
    //             uuid: Uuid::now_v6(&node),
    //             title: "Nobody".into(),
    //             created: now,
    //             complete_by: None,
    //             status: TaskStatus::New,
    //             percentage: Some(90),
    //         },
    //     ];
    //     let stringyfied_json = serde_json::to_string_pretty(&tasks)?;
    //     let mut file = File::create(".\\resources\\sample_task.json")?;
    //     file.write_all(stringyfied_json.as_bytes())?;
    //     file.flush()?;
    // }

    //TODO: use env for filepath
    let mut app = App::new(Some(PathBuf::from(".\\resources\\sample_task.json")));

    'main_loop: loop {
        terminal.draw(|frame| ui(frame, &app))?;

        let exit = handle_event_loop(&mut app, &terminal.get_frame())?;
        if exit {
            break 'main_loop;
        }
    }

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}

fn handle_event_loop(app: &mut App, frame: &Frame) -> Result<bool> {
    let mut exit = false;

    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                // Exit application (for now)
                if key.code == KeyCode::Char('c') {
                    exit = true;
                }
            } else {
                match key.code {
                    KeyCode::Tab => match app.get_selected_panel() {
                        SelectedPanel::Task(Some((_, task_field)))
                            if task_field != SelectedTaskField::Description =>
                        {
                            app.select_next_task_field()
                        }
                        _ => app.select_next_panel(),
                    },
                    KeyCode::BackTab => match app.get_selected_panel() {
                        SelectedPanel::Task(Some((_, task_field)))
                            if task_field != SelectedTaskField::Status =>
                        {
                            app.select_previous_task_field()
                        }
                        _ => app.select_previous_panel(),
                    },

                    KeyCode::Up | KeyCode::Down => {
                        if let SelectedPanel::List(Some(_)) = app.get_selected_panel() {
                            if key.code == KeyCode::Up {
                                app.decrement_list_selected_index();
                            } else {
                                app.increment_list_selected_index();
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(exit)
}
