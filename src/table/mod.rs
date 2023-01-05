pub mod create_tasks_table;

use crate::{
    constants::{DEFAULT_TABLE_CHARS, MAX_STATES_NUMBER, MIN_STATES_NUMBER},
    task::{Direction, Task},
};
use std::io::{prelude::*, Error, ErrorKind};

#[derive(Debug, Clone)]
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

    pub fn new_from_buffer(buffer: &mut impl BufRead) -> Result<Self, Error> {
        let mut table = Self::new_empty();
        let mut lines_iter = buffer.lines();

        let first_line = lines_iter
            .next()
            .ok_or(Error::from(ErrorKind::UnexpectedEof))??
            .filter_characters();

        table.set_characters(&first_line);

        for (task_state, line) in lines_iter.enumerate() {
            let line = line?;
            let mut task_data_iterator = line.split_whitespace().array_chunks::<3>();

            table.set_states_number(task_state + 1);

            for task_character in first_line.chars() {
                let [state, character, direction] = task_data_iterator
                    .next()
                    .ok_or(Error::from(ErrorKind::InvalidData))?;

                let state: usize = state.parse().or(Err(Error::from(ErrorKind::InvalidData)))?;
                let character = character.chars().next().unwrap();
                let direction: Direction = direction
                    .chars()
                    .next()
                    .unwrap()
                    .try_into()
                    .or(Err(Error::from(ErrorKind::InvalidData)))?;

                let task = Task {
                    state,
                    character,
                    direction,
                };

                table.set_task_by_state_and_character(task, task_state, task_character);
            }
        }

        Ok(table)
    }

    pub fn write_to_buffer(&self, buffer: &mut impl Write) -> Result<(), Error> {
        let mut line = self
            .characters
            .chars()
            .fold(String::new(), |acc, c| acc + &format!("      {c}   "));

        writeln!(buffer, "{}", &line[4..])?;

        for row in &self.tasks {
            line.clear();

            for task in row {
                line += &format!(
                    "    {:0>2} {} {}",
                    task.state, task.character, task.direction
                );
            }

            writeln!(buffer, "{}", &line[4..])?;
        }

        Ok(())
    }

    pub fn get_task(&self, state: usize, character: char) -> Option<&Task> {
        let char_index = self
            .sorted_characters
            .iter()
            .position(|c| *c == character)?;

        self.tasks.get(state)?.get(char_index)
    }

    pub fn set_task_by_position(&mut self, task: Task, row: usize, column: usize) {
        self.tasks[row][column] = task
    }

    pub fn set_task_by_state_and_character(
        &mut self,
        task: Task,
        state: usize,
        character: char,
    ) -> Option<()> {
        let char_index = self
            .sorted_characters
            .iter()
            .position(|c| *c == character)?;

        *self.tasks.get_mut(state)?.get_mut(char_index)? = task;
        Some(())
    }

    pub fn get_characters(&self) -> &String {
        &self.characters
    }

    pub fn set_characters(&mut self, new_characters: &String) {
        let filtered_new_characters = new_characters.filter_characters();

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

trait FilterCharacters {
    /// Return string without duplicated characters and whitespaces
    fn filter_characters(&self) -> String;
}

impl FilterCharacters for str {
    fn filter_characters(&self) -> String {
        self.chars().fold("".to_string(), |acc, c| {
            if !acc.contains(c) && !c.is_whitespace() {
                acc + &c.to_string()
            } else {
                acc
            }
        })
    }
}
