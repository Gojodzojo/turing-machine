use crate::{
    constants::{
        MACHINE_SELF_TIMER_INTERVAL_STEP, MIN_MACHINE_SELF_TIMER_INTERVAL,
        STOP_MACHINE_SELF_TIMER_VALUE,
    },
    table::create_tasks_table::create_tasks_table,
    tape::create_tape_preview::create_tape_preview,
    App, Message,
};
use iced::{
    widget::{button, column as ui_column, slider, text},
    Element, Length,
};

use super::scene_frame;

pub fn machine_scene<'a>(app: &App) -> Element<'a, Message> {
    let left_column = left_column(app);

    let tasks_table = create_tasks_table(
        &app.table,
        false,
        app.machine.get_tape().get_current_char(),
        app.machine.get_state(),
    );

    let tape_preview = create_tape_preview(app.machine.get_tape());

    scene_frame(tape_preview.into(), left_column.into(), tasks_table.into())
}

fn left_column<'a>(app: &App) -> Element<'a, Message> {
    let stop_button = button("Stop")
        .padding(10)
        .width(Length::Fill)
        .on_press(Message::MachineStopped);

    let step = text(format!("Step: {}", app.machine.get_step()));
    let state = text(format!("State: {}", app.machine.get_state()));

    let next_step_button: Element<_> = if app.machine.is_halted() {
        text("Machine halted").into()
    } else {
        let slider_val = if let Some(interval) = app.machine.get_self_timer_interval() {
            interval.as_millis() as u32
        } else {
            STOP_MACHINE_SELF_TIMER_VALUE
        };

        let on_slider_change = |v: u32| {
            let c = match v {
                STOP_MACHINE_SELF_TIMER_VALUE => None,
                v => Some(v),
            };
            Message::MachineSelfTimerIntervalChange(c)
        };

        let slider_label_text = match slider_val {
            STOP_MACHINE_SELF_TIMER_VALUE => "Self-timer interval: None".to_string(),
            v => format!("Self-timer interval: {}ms", v),
        };

        ui_column![
            text(slider_label_text),
            slider(
                MIN_MACHINE_SELF_TIMER_INTERVAL..=STOP_MACHINE_SELF_TIMER_VALUE,
                slider_val,
                on_slider_change
            )
            .step(MACHINE_SELF_TIMER_INTERVAL_STEP),
            button("Next step")
                .padding(10)
                .width(Length::Fill)
                .on_press(Message::MachineNextStep),
        ]
        .into()
    };

    ui_column![step, state, next_step_button, stop_button]
        .width(Length::Units(200))
        .spacing(10)
        .into()
}
