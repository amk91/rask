use std::{fs::File, io::Read, path::PathBuf};
use anyhow::{anyhow, Result};
use chrono::offset;
use rand::Rng;

pub mod task;
use task::{Task, TaskStatus};
use uuid::Uuid;

pub enum SelectedPanel {
    None,
    List(Option<usize>),
    Title(Option<(usize, Uuid)>),
    Task(Option<(usize, Uuid)>),
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

    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn get_tasks_titles(&self) -> Vec<String> {
        self.tasks.iter().map(|task| task.title.clone()).collect()
    }

    pub fn get_task_by_uuid(&self, uuid: Uuid) -> Option<&Task> {
        self.tasks.iter().find(|task| task.uuid == uuid)
    }

    pub fn get_task_by_index(&self, index: usize) -> Option<&Task> {
        self.tasks.get(index)
    }

    pub fn get_tasks_size(&self) -> usize {
        self.tasks.len()
    }

    pub fn get_selected_panel(&self) -> &SelectedPanel {
        &self.selected_panel
    }

    pub fn get_list_selected_index(&self) -> Option<usize> {
        if let SelectedPanel::List(Some(index)) = self.selected_panel {
            return Some(index);
        }

        None
    }

    pub fn increment_list_selected_index(&mut self) {
        if let SelectedPanel::List(Some(ref mut index)) = self.selected_panel {
            *index = (*index + 1) % self.tasks.len();
        }
    }

    pub fn decrement_list_selected_index(&mut self) {
        if let SelectedPanel::List(Some(ref mut index)) = self.selected_panel {
            let result = *index as isize - 1;
            if result < 0 {
                *index = self.tasks.len() - 1;
            } else {
                *index = result as usize;
            }
        }
    }

    pub fn select_next_panel(&mut self) {
        match self.selected_panel {
            SelectedPanel::None => self.selected_panel = SelectedPanel::List(
                if self.tasks.len() > 0 { Some(0) } else { None }
            ),
            SelectedPanel::List(value) => self.selected_panel = SelectedPanel::Title(
                if let Some(index) = value { Some((index, self.tasks[index].uuid)) } else { None }
            ),
            SelectedPanel::Title(value) => self.selected_panel = SelectedPanel::Task(value),
            SelectedPanel::Task(value) => self.selected_panel = SelectedPanel::List(
                if let Some((index, _)) = value { Some(index) } else { None }
            ),
        }
    }

    pub fn select_previous_panel(&mut self) {
        match self.selected_panel {
            SelectedPanel::None => self.selected_panel = SelectedPanel::List(
                if self.tasks.len() > 0 { Some(0) } else { None }
            ),
            SelectedPanel::List(value) => self.selected_panel = SelectedPanel::Task(
                if let Some(index) = value { Some((index, self.tasks[index].uuid))} else { None }
            ),
            SelectedPanel::Task(value) => self.selected_panel = SelectedPanel::Title(value),
            SelectedPanel::Title(value) => self.selected_panel = SelectedPanel::List(
                if let Some((index, _)) = value { Some(index) } else { None }
            ),
        }
    }
}
