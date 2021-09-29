use std::borrow::Cow;
use std::cell::RefCell;

use deno_core::error::AnyError;
use deno_core::Resource;
use deno_core::ResourceId;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::platform::run_return::EventLoopExtRunReturn;

use crate::check_unstable;
use crate::event::Event;

pub struct EventLoopResource(pub RefCell<EventLoop<()>>);

impl Resource for EventLoopResource {
  fn name(&self) -> Cow<str> {
    "eventLoop".into()
  }
}

pub fn op_windowing_event_loop_new(
  state: &mut deno_core::OpState,
  _args: (),
  _zero_copy: (),
) -> Result<ResourceId, AnyError> {
  check_unstable(state, "Deno.windowing.EventLoop");

  Ok(
    state
      .resource_table
      .add(EventLoopResource(RefCell::new(EventLoop::new()))),
  )
}

pub fn op_windowing_event_loop_step(
  state: &mut deno_core::OpState,
  rid: ResourceId,
  _zero_copy: (),
) -> Result<Vec<Event>, AnyError> {
  let event_loop = state
    .resource_table
    .get::<EventLoopResource>(rid)?;
  let mut events = Vec::new();

  event_loop
    .0
    .borrow_mut()
    .run_return(|event, _, control_flow| {
      *control_flow = ControlFlow::Exit;
      events.push(Event::from(event));
    });

  Ok(events)
}
