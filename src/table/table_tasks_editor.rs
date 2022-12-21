use iced::{
    alignment,
    widget::{
        column as ui_column, container, horizontal_rule, row, text, text_input as txt_input,
        vertical_rule, Container, Row,
    },
    Alignment, Element, Length,
};

use crate::{
    constants::{DEFAULT_STATE, EMPTY_CHAR},
    task::{Direction, Task},
};

use super::Table;

const CELL_HEIGHT: u16 = 50;
const CELL_WIDTH: u16 = 125;

pub fn table_tasks_editor<'a, Message: 'a + Clone>(
    table: &Table,
    on_task_change: &'a impl Fn(Task, usize, usize) -> Message,
) -> Row<'a, Message> {
    let mut sorted_old_characters: Vec<_> = table.characters.chars().collect();
    sorted_old_characters.sort();

    let mut tasks_editor: Row<Message> =
        row![vertical_rule(0)]
            .align_items(Alignment::Fill)
            .width(Length::Units(
                (1 + sorted_old_characters.len() as u16) * CELL_WIDTH,
            ));

    let mut first_column = ui_column![
        horizontal_rule(0),
        task_editor_cell(vec![text(" ").into()]),
        horizontal_rule(0)
    ]
    .align_items(Alignment::Center)
    .width(Length::FillPortion(1));

    for i in 0..table.states_number {
        first_column = first_column
            .push(task_editor_cell(vec![text(i).into()]))
            .push(horizontal_rule(0));
    }

    tasks_editor = tasks_editor.push(first_column).push(vertical_rule(0));

    for (column_index, char) in sorted_old_characters.iter().enumerate() {
        let mut col = ui_column![
            horizontal_rule(0),
            task_editor_cell(vec![text(char).into()]),
            horizontal_rule(0)
        ]
        .align_items(Alignment::Center)
        .width(Length::FillPortion(1));

        for row_index in 0..table.tasks.len() {
            let Task {
                state,
                character,
                direction,
            } = table.tasks[row_index][column_index];

            let update_state = move |state_str: String| {
                let state: usize = if state_str.len() == 0 {
                    DEFAULT_STATE
                } else if state_str.len() > 2 {
                    state_str[0..2].parse().unwrap_or(state)
                } else {
                    state_str.parse().unwrap_or(state)
                };
                let task = Task {
                    character,
                    direction,
                    state,
                };
                on_task_change(task, row_index, column_index)
            };

            let update_char = move |char_str: String| {
                let character = if char_str.len() == 0 {
                    EMPTY_CHAR
                } else {
                    char_str.chars().last().unwrap()
                };
                let task = Task {
                    character,
                    direction,
                    state,
                };
                on_task_change(task, row_index, column_index)
            };

            let update_direction = move |direction_str: String| {
                let direction = if direction_str.len() == 0 {
                    Direction::Stop
                } else {
                    direction_str
                        .chars()
                        .last()
                        .unwrap()
                        .try_into()
                        .unwrap_or(direction)
                };
                let task = Task {
                    character,
                    direction,
                    state,
                };
                on_task_change(task, row_index, column_index)
            };

            col = col.push(task_editor_cell(vec![
                txt_input("state", &format!("{}", state), update_state)
                    .width(Length::Units(20))
                    .into(),
                txt_input("char", &format!("{}", character), update_char)
                    .width(Length::Units(10))
                    .into(),
                txt_input("direction", &format!("{}", direction), update_direction)
                    .width(Length::Units(10))
                    .into(),
            ]));
            col = col.push(horizontal_rule(0));
        }

        tasks_editor = tasks_editor.push(col);
        tasks_editor = tasks_editor.push(vertical_rule(0));
    }

    tasks_editor
}

fn task_editor_cell<'a, Message: 'a + Clone>(
    children: Vec<Element<'a, Message>>,
) -> Container<'a, Message> {
    container(Row::with_children(children).spacing(5))
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .height(Length::Units(CELL_HEIGHT))
}
