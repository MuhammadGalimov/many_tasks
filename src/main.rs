mod task;
mod state;
mod project;

use druid::widget::prelude::*;
use druid::widget::{Flex, Label, TextBox, Button, List};
use druid::{AppLauncher, WindowDesc, Widget, PlatformError, WidgetExt};

use task::Task;

fn build_ui() -> impl Widget<Task> {
    let label = Label::new(|data: &Task, _env: &Env| {
        // clone() - ?
        data.title.clone()
    });

    let text_box  = TextBox::new()
        .with_placeholder("Enter task title")
        .lens(Task::title);

    // let save_button = Button::new("Save")
    //     .on_click(|_ctx, data: &Task, _env| {

    //     });

    Flex::column()
        .with_child(label)
        .with_child(text_box)
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(|| { build_ui() })
        .title("Many Tasks");

    let initial_state = Task::new("hmhm");

    AppLauncher::with_window(main_window)
        .launch(initial_state)?;

    Ok(())
}
