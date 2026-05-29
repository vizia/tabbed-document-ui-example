mod storage;
mod ui;
mod worker;

fn main() -> Result<(), vizia::ApplicationError> {
    ui::run()
}
