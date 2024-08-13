pub trait Controller {
    fn advance_tick(&self) -> Result<(), String>;
    fn draw(&self);
}

pub struct ControllerImpl {}

impl Controller for ControllerImpl {
    fn draw(&self) {
        println!("Draw controller");
    }

    fn advance_tick(&self) -> Result<(), String> {
        println!("Advance tick");
        Ok(())
    }
}
