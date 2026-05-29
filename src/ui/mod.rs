use vizia::prelude::*;

mod bootstrap;
pub mod commands;
pub mod dialogs;
pub mod effects;
pub mod event_handling;
pub mod events;
pub mod model;
mod shell;
pub mod views;

pub fn run() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let app = bootstrap::bootstrap_app(cx);
        shell::build_app_shell(cx, app);
    })
    .title("Tabbed Document UI")
    .inner_size((800, 600))
    .run()
}
