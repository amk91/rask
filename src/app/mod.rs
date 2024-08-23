use std::{fs::File, io::Read, path::PathBuf};
use anyhow::{anyhow, Result};
use chrono::offset;
use rand::Rng;

pub mod task;
use task::{Task, TaskStatus};
use uuid::Uuid;

pub enum SelectedPanel {
    None,
    List(usize),
    Title((usize, Uuid)),
    Task((usize, Uuid)),
}

pub struct App {
    uuid_node: [u8; 6],
    tasks: Vec<Task>,
    selected_panel: SelectedPanel,
}

impl App {
    pub fn new(filepath: Option<PathBuf>) -> App {
        App {
            uuid_node: rand::thread_rng().gen(),
            tasks: match filepath {
                Some(filepath) => App::load_tasks_from_file(filepath).unwrap_or(vec![]),
                None => vec![]
            },
            selected_panel: SelectedPanel::None,
        }
    }

    pub fn load_tasks_from_file(filepath: PathBuf) -> Result<Vec<Task>> {
        if filepath.exists() {
            let mut file = File::open(&filepath)?;
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)?;

            return match serde_json::from_str(&buffer) {
                Ok(tasks) => Ok(tasks),
                Err(err) => Err(anyhow!(
                    "Unable to parse tasks list from file {} [[{}]]",
                    filepath.to_str().unwrap_or("N/A"),
                    err
                )),
            }
        }

        Err(anyhow!("File {} does not exist", filepath.to_str().unwrap_or("N/A")))
    }

    pub fn add_task(&mut self, title: String) -> &Task {
        self.tasks.push(Task::new(
            &self.uuid_node,
            title,
            offset::Utc::now(),
            None,
            None,
            TaskStatus::New,
            None,
        ));
        self.tasks.last().expect(&format!(""))
    }

    pub fn remove_task(&mut self, uuid: Uuid) {
        if let Some(index) = self.tasks.iter().position(|task| task.uuid == uuid) {
            self.tasks.remove(index);
        }
    }

    pub fn get_tasks_titles(&self) -> Vec<String> {
        self.tasks.iter().map(|task| task.title.clone()).collect()
    }

    pub fn get_task(&self, uuid: Uuid) -> Option<&Task> {
        self.tasks.iter().find(|task| task.uuid == uuid)
    }

    pub fn get_selected_panel(&self) -> &SelectedPanel {
        &self.selected_panel
    }

    pub fn select_next_panel(&mut self) {
        match self.selected_panel {
            SelectedPanel::None => self.selected_panel = SelectedPanel::List(0),
            SelectedPanel::List(index) => {
                self.selected_panel = SelectedPanel::Title((index, self.tasks[index].uuid))
            },
            SelectedPanel::Title((index, uuid)) => self.selected_panel = SelectedPanel::Task((index, uuid)),
            SelectedPanel::Task((index, _)) => self.selected_panel = SelectedPanel::List(index),
        }
    }

    pub fn select_previous_panel(&mut self) {
        match self.selected_panel {
            SelectedPanel::None => self.selected_panel = SelectedPanel::List(0),
            SelectedPanel::List(index) => {
                self.selected_panel = SelectedPanel::Task((index, self.tasks[index].uuid))
            },
            SelectedPanel::Task((index, uuid)) => self.selected_panel = SelectedPanel::Title((index, uuid)),
            SelectedPanel::Title((index, _)) => self.selected_panel = SelectedPanel::List(index),
        }
    }
}
