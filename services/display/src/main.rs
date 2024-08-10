use std::{sync::{Arc, Mutex}, thread};

use controller::controller::Controller;
use domain::repository::Repository;
use presenter::presenter::Presenter;

pub mod domain;
pub mod controller;
pub mod presenter;

fn main() {

    let repository = Arc::new(Mutex::new(Repository::new()));

    let repo = Arc::clone(&repository);

    let controller_thread = thread::spawn(move || {
        let controller = Controller::new("1234", repo);
        controller.run ();
    });

    let repo = Arc::clone(&repository);

    let presenter_thread = thread::spawn(move || {
        let presenter = Presenter::new(repo);

        presenter.run();
    });

    controller_thread.join().unwrap();
    presenter_thread.join().unwrap();
}
