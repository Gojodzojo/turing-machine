pub mod table_tasks_editor;

use crate::{
    constants::{DEFAULT_TABLE_CHARS, MAX_STATES_NUMBER, MIN_STATES_NUMBER},
    task::Task,
};

pub struct Table {
    //// Number of possible states
    states_number: usize,

    /// Characters used to execute the right task.
    /// They are not sorted.
    characters: String,

    /// The same as characters, but sorted
    sorted_characters: Vec<char>,

    /// Tasks to execute for certain state and character.
    /// The first index is number of state and
    /// the second one is index of certain character
    /// when characters string is sorted.
    tasks: Vec<Vec<Task>>,
}

impl Table {
    pub fn new_empty() -> Self {
        let states_number = 5;
        let characters: String = DEFAULT_TABLE_CHARS.to_string();
        let mut sorted_characters: Vec<_> = characters.chars().collect();
        sorted_characters.sort();
        let tasks: Vec<Vec<Task>> = (0..states_number)
            .map(|_| (0..characters.len()).map(|_| Task::new()).collect())
            .collect();

        Self {
            states_number,
            characters,
            sorted_characters,
            tasks,
        }
    }

    pub fn get_task(&self, state: usize, character: char) -> Option<&Task> {
        let char_index = self
            .sorted_characters
            .iter()
            .position(|c| *c == character)?;

        self.tasks.get(state)?.get(char_index)
    }

    pub fn set_task(&mut self, task: Task, row: usize, column: usize) {
        self.tasks[row][column] = task
    }

    pub fn get_characters(&self) -> &String {
        &self.characters
    }

    pub fn set_characters(&mut self, new_characters: String) {
        // Return string without duplicated characters
        let filtered_new_characters = new_characters.chars().fold("".to_string(), |acc, c| {
            if !acc.contains(c) {
                acc + &c.to_string()
            } else {
                acc
            }
        });

        let removed_characters: String = self
            .characters
            .chars()
            .filter_map(|c| {
                if filtered_new_characters.contains(c) {
                    None
                } else {
                    Some(c)
                }
            })
            .collect();

        let added_characters: String = filtered_new_characters
            .chars()
            .filter_map(|c| {
                if self.characters.contains(c) {
                    None
                } else {
                    Some(c)
                }
            })
            .collect();

        for removed_char in removed_characters.chars() {
            let removed_char_index = self
                .sorted_characters
                .iter()
                .position(|c| *c == removed_char)
                .unwrap();

            for state_index in 0..self.states_number {
                self.tasks[state_index].remove(removed_char_index);
            }

            self.sorted_characters.remove(removed_char_index);
        }

        for added_char in added_characters.chars() {
            let added_char_index = self
                .sorted_characters
                .iter()
                .position(|c| *c > added_char)
                .unwrap_or(self.sorted_characters.len());

            for state_index in 0..self.states_number {
                self.tasks[state_index].insert(added_char_index, Task::new());
            }

            self.sorted_characters.insert(added_char_index, added_char);
        }

        self.characters = filtered_new_characters;
    }

    pub fn get_states_number(&self) -> usize {
        self.states_number
    }

    pub fn set_states_number(&mut self, new_states_number: usize) {
        let new_states_number = if new_states_number <= MIN_STATES_NUMBER {
            MIN_STATES_NUMBER
        } else if new_states_number >= MAX_STATES_NUMBER {
            MAX_STATES_NUMBER
        } else {
            new_states_number
        };

        if new_states_number < self.states_number {
            self.tasks.drain(new_states_number..);
        } else {
            for _ in self.states_number..new_states_number {
                self.tasks
                    .push((0..self.characters.len()).map(|_| Task::new()).collect())
            }
        }

        self.states_number = new_states_number;
    }
}
