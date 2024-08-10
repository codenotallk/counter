use controller::controller::Controller;

mod domain;
mod controller;

fn main() {
    let mut controller = Controller::new ("127.0.0.1:34254", "127.0.0.1:1234");

    controller.run ();
}
