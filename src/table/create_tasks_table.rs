use iced::{
    alignment, theme,
    widget::{
        self, column as ui_column, container, horizontal_rule, row, text, text_input as txt_input,
        vertical_rule, Container, Row,
    },
    Alignment, Background, Color, Element, Length, Theme,
};

use crate::{
    constants::{DEFAULT_STATE, EMPTY_CHAR},
    task::{Direction, Task},
};

use super::Table;

const CELL_HEIGHT: u16 = 50;
const CELL_WIDTH: u16 = 125;

pub fn create_tasks_table<'a, Message: 'a + Clone>(
    table: &Table,
    on_task_change: &'a impl Fn(Task, usize, usize) -> Message,
    is_mutable: bool,
    selected_column: char,
    selected_row: usize,
) -> Row<'a, Message> {
    let mut tasks_table: Row<Message> =
        row![vertical_rule(0)]
            .align_items(Alignment::Fill)
            .width(Length::Units(
                (1 + table.sorted_characters.len() as u16) * CELL_WIDTH,
            ));

    let mut first_column = ui_column![
        horizontal_rule(0),
        table_cell(vec![text(" ").into()], false),
        horizontal_rule(0)
    ]
    .align_items(Alignment::Center)
    .width(Length::FillPortion(1));

    for i in 0..table.states_number {
        first_column = first_column
            .push(table_cell(vec![text(i).into()], false))
            .push(horizontal_rule(0));
    }

    tasks_table = tasks_table.push(first_column).push(vertical_rule(0));

    for (column_index, char) in table.sorted_characters.iter().enumerate() {
        let mut col = ui_column![
            horizontal_rule(0),
            table_cell(vec![text(char).into()], false),
            horizontal_rule(0)
        ]
        .align_items(Alignment::Center)
        .width(Length::FillPortion(1));

        for row_index in 0..table.tasks.len() {
            let task = table.tasks[row_index][column_index];
            let is_selected = selected_column == *char && selected_row == row_index;

            let cell = if is_mutable {
                mutable_cell(task, row_index, column_index, is_selected, on_task_change)
            } else {
                unmutable_cell(task, is_selected)
            };

            col = col.push(cell).push(horizontal_rule(0));
        }

        tasks_table = tasks_table.push(col).push(vertical_rule(0));
    }

    tasks_table
}

fn unmutable_cell<'a, Message: 'a + Clone>(
    Task {
        state,
        character,
        direction,
    }: Task,
    is_selected: bool,
) -> Container<'a, Message> {
    table_cell(
        vec![
            text(state).width(Length::Units(10)).into(),
            text(character).width(Length::Units(10)).into(),
            text(direction).width(Length::Units(10)).into(),
        ],
        is_selected,
    )
}

fn mutable_cell<'a, Message: 'a + Clone>(
    Task {
        state,
        character,
        direction,
    }: Task,
    row_index: usize,
    column_index: usize,
    is_selected: bool,
    on_task_change: &'a impl Fn(Task, usize, usize) -> Message,
) -> Container<'a, Message> {
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

    table_cell(
        vec![
            txt_input("state", &format!("{}", state), update_state)
                .width(Length::Units(20))
                .into(),
            txt_input("char", &format!("{}", character), update_char)
                .width(Length::Units(10))
                .into(),
            txt_input("direction", &format!("{}", direction), update_direction)
                .width(Length::Units(10))
                .into(),
        ],
        is_selected,
    )
}

fn table_cell<'a, Message: 'a + Clone>(
    children: Vec<Element<'a, Message>>,
    is_selected: bool,
) -> Container<'a, Message> {
    let theme = if is_selected {
        let f: fn(&Theme) -> container::Appearance = |_| widget::container::Appearance {
            background: Some(Background::Color(Color::from_rgba8(0, 0, 0, 0.3))),
            ..Default::default()
        };
        theme::Container::from(f)
    } else {
        theme::Container::default()
    };

    container(Row::with_children(children).spacing(5))
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .height(Length::Units(CELL_HEIGHT))
        .width(Length::Fill)
        .style(theme)
}
