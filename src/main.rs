use iced::{Sandbox, Settings};

mod sidebar;
pub mod window;
mod pages;

fn main() {
    let res = window::MainWindow::run(Settings::default());

    if res.is_err() {
        println!("{}", res.unwrap_err());
    }
}