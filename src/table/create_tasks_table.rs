use iced::{
    alignment,
    theme::{self, palette::Pair},
    widget::{
        self, column as ui_column, container, horizontal_rule, row, scrollable as ui_scrollable,
        text, vertical_rule, Container, Row,
    },
    Alignment, Background, Element, Length, Theme,
};
use iced_native::widget::{scrollable, Id};

use crate::{
    constants::{DEFAULT_STATE, EMPTY_CHAR},
    task::{Direction, Task},
    Message,
};

use super::{blankable_input::blankable_input, Table};

const CELL_HEIGHT: u16 = 40;
const CELL_WIDTH: u16 = 125;

pub fn create_tasks_table<'a>(
    table: &Table,
    is_mutable: bool,
    selected_column: char,
    selected_row: usize,
    focused_widget: &'a Option<Id>,
) -> Element<'a, Message> {
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
                let on_task_change = move |task: Task| -> Message {
                    Message::TableTaskChanged(task, row_index, column_index)
                };
                mutable_cell(task, is_selected, focused_widget, on_task_change)
            } else {
                immutable_cell(task, is_selected)
            };

            col = col.push(cell).push(horizontal_rule(0));
        }

        tasks_table = tasks_table.push(col).push(vertical_rule(0));
    }

    ui_scrollable(tasks_table)
        .horizontal_scroll(scrollable::Properties::default())
        .into()
}

fn immutable_cell<'a>(
    Task {
        state,
        character,
        direction,
    }: Task,
    is_selected: bool,
) -> Container<'a, Message> {
    table_cell(
        vec![
            text(state).width(Length::Units(20)).into(),
            text(character).width(Length::Units(20)).into(),
            text(direction).width(Length::Units(10)).into(),
        ],
        is_selected,
    )
}

fn mutable_cell<'a, F: 'a + Clone + Fn(Task) -> Message>(
    Task {
        state,
        character,
        direction,
    }: Task,
    is_selected: bool,
    focused_widget: &'a Option<Id>,
    on_task_change: F,
) -> Container<'a, Message> {
    let c = on_task_change.clone();
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
        c(task)
    };

    let c = on_task_change.clone();
    let update_char = move |char_str: String| {
        let character = match char_str.chars().last() {
            None => EMPTY_CHAR,
            Some(c) if c.is_whitespace() => EMPTY_CHAR,
            Some(c) => c,
        };

        let task = Task {
            character,
            direction,
            state,
        };
        c(task)
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
        on_task_change(task)
    };

    table_cell(
        vec![
            blankable_input(
                "St",
                format!("{}", state),
                focused_widget,
                Length::Units(20),
                update_state,
            )
            .into(),
            blankable_input(
                "C",
                format!("{}", character),
                focused_widget,
                Length::Units(20),
                update_char,
            )
            .into(),
            blankable_input(
                "D",
                format!("{}", direction),
                focused_widget,
                Length::Units(10),
                update_direction,
            )
            .into(),
        ],
        is_selected,
    )
}

fn table_cell<'a>(
    children: Vec<Element<'a, Message>>,
    is_selected: bool,
) -> Container<'a, Message> {
    let theme = if is_selected {
        let f: fn(&Theme) -> container::Appearance = |t| {
            let Pair { color, text } = t.extended_palette().background.strong;

            widget::container::Appearance {
                background: Some(Background::Color(color)),
                text_color: Some(text),
                ..Default::default()
            }
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
