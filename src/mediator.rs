// use std::rc::Rc;
//
// pub trait Mediator {
//   fn create_colleague(&mut self);
//   fn colleague_changed(&mut self);
// }
//
// pub trait Colleague {
//   fn set_mediator(&mut self, mediator: &dyn Mediator);
//   fn set_colleague_enabled(&mut self, enabled: bool);
// }
//
// pub struct UserAccount {
//   name: String,
//   mediator: Rc<dyn Mediator>,
// }
//
// impl Colleague for UserAccount {
//   fn set_mediator(&mut self, mediator: &dyn Mediator) {
//     self.mediator = Rc::new(mediator);
//   }
//
//   fn set_colleague_enabled(&mut self, enabled: bool) {
//     todo!()
//   }
// }
