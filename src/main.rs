use std::collections::HashMap;

use iced::alignment;
use iced::theme;
use iced::widget::button;
use iced::widget::horizontal_rule;
use iced::widget::pane_grid;
use iced::widget::vertical_rule;
use iced::widget::{
    checkbox, column as ui_column, container, horizontal_space, image, radio, row, scrollable,
    slider, text, text_input, toggler, vertical_space,
};
use iced::widget::{Button, Column, Container, Slider};
use iced::Alignment;
use iced::{Color, Element, Length, Renderer, Sandbox, Settings};

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Stop,
}

struct Task {
    state: usize,
    char: char,
    direction: Direction,
}

impl Task {
    pub fn new() -> Self {
        Self {
            state: 0,
            char: '0',
            direction: Direction::Stop,
        }
    }
}

const EMPTY_CHAR: char = '#';

struct Table {
    states_number: usize,
    characters: String,
    tasks: Vec<Vec<Task>>,
}

impl Table {
    pub fn new_empty() -> Self {
        let states_number = 5;
        let characters: String = format!("{}{}", "abcde", EMPTY_CHAR);
        let tasks: Vec<Vec<Task>> = (0..states_number)
            .map(|_| (0..characters.len()).map(|_| Task::new()).collect())
            .collect();

        Self {
            states_number,
            characters,
            tasks,
        }
    }
}

struct RunningMachine {
    state: usize,
    tape: String,
    cursor_position: isize,
}

impl RunningMachine {
    pub fn new(initial_tape: String, initial_cursor_position: isize) -> Self {
        Self {
            state: 0,
            tape: initial_tape,
            cursor_position: initial_cursor_position,
        }
    }
}

struct App {
    table: Table,
    initial_tape: String,
    innitial_cursor_position: isize,
    running_machine: RunningMachine,
}

#[derive(Debug, Clone)]
enum Message {
    InitialTapeChanged(String),
    InitialCursorPositionChanged(String),
    InitialCursorPositionIncremented,
    InitialCursorPositionDecremented,
    TableCharactersChanged(String),
    TableStatesNumberChanged(String),
    TableStatesNumberIncremented,
    TableStatesNumberDecremented,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        let table = Table::new_empty();
        let initial_tape = String::new();
        let innitial_cursor_position = 0;
        let running_machine = RunningMachine::new(initial_tape.clone(), innitial_cursor_position);

        Self {
            table,
            initial_tape,
            innitial_cursor_position,
            running_machine,
        }
    }

    fn title(&self) -> String {
        "Turing Machine".into()
    }

    fn update(&mut self, message: Self::Message) {}

    fn view(&self) -> Element<Self::Message> {
        let initial_tape_input = text_input(
            "Set initial tape...",
            &self.initial_tape,
            Message::InitialTapeChanged,
        )
        .padding(10)
        .size(20);

        let initial_cursor_position_input = text_input(
            "Set initial cursor position...",
            &format!("{}", self.innitial_cursor_position),
            Message::InitialCursorPositionChanged,
        )
        .padding(10)
        .size(20);

        let initial_cursor_position_increment_button = button("+")
            .padding(10)
            .on_press(Message::InitialCursorPositionIncremented);

        let initial_cursor_position_decrement_button = button("-")
            .padding(10)
            .on_press(Message::InitialCursorPositionDecremented);

        let table_characters_input = text_input(
            "Set table characters...",
            &self.table.characters,
            Message::TableCharactersChanged,
        )
        .padding(10)
        .size(20);

        let table_states_number_input = text_input(
            "Set table states number...",
            &format!("{}", self.table.states_number),
            Message::TableStatesNumberChanged,
        )
        .padding(10)
        .size(20);

        let table_states_number_increment_button = button("+")
            .padding(10)
            .on_press(Message::TableStatesNumberIncremented);

        let table_states_number_decrement_button = button("-")
            .padding(10)
            .on_press(Message::TableStatesNumberDecremented);

        let mut table = row![vertical_rule(0)].align_items(Alignment::Fill);

        let mut first_column = ui_column![horizontal_rule(0), " ", horizontal_rule(0)]
            .align_items(Alignment::Center)
            .width(Length::FillPortion(1));

        for i in 0..self.table.tasks.len() {
            first_column = first_column.push(text(i));
            first_column = first_column.push(horizontal_rule(0));
        }

        table = table.push(first_column);
        table = table.push(vertical_rule(0));

        for (column_index, char) in self.table.characters.char_indices() {
            let mut col = ui_column![horizontal_rule(0), text(char), horizontal_rule(0)]
                .align_items(Alignment::Center)
                .width(Length::FillPortion(1));

            for row_index in 0..self.table.tasks.len() {
                let Task {
                    state,
                    char,
                    direction,
                } = self.table.tasks[row_index][column_index];

                col = col.push(
                    container(text(format!("{} {:?} {}", state, direction, char))).padding(10),
                );
                col = col.push(horizontal_rule(0));
            }

            table = table.push(col);
            table = table.push(vertical_rule(0));
        }

        let content = ui_column![
            "Tape text",
            initial_tape_input,
            "Cursor position",
            row![
                initial_cursor_position_input,
                initial_cursor_position_increment_button,
                initial_cursor_position_decrement_button,
            ]
            .spacing(10),
            "Table states number",
            row![
                table_states_number_input,
                table_states_number_increment_button,
                table_states_number_decrement_button,
            ]
            .spacing(10),
            "Table characters",
            table_characters_input,
            table
        ]
        .spacing(20)
        .padding(20)
        .max_width(600);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
