use std::{sync::mpsc::{self, Receiver, Sender}, thread};

use controller::controller::Controller;
use domain::counter::Counter;
use presenter::presenter::Presenter;

pub mod domain;
pub mod controller;
pub mod presenter;

fn main() {
    
    let (tx, rx): (Sender<Counter>, Receiver<Counter>) = mpsc::channel();

    let controller_thread = thread::spawn(move || {
        let controller = Controller::new("1234", tx);
        controller.run ();
    });

    let presenter_thread = thread::spawn(move || {
        let presenter = Presenter::new(rx);

        presenter.run();
    });

    controller_thread.join().unwrap();
    presenter_thread.join().unwrap();
}
