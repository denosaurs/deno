use std::borrow::Cow;

use deno_core::error::AnyError;
use deno_core::serde::Deserialize;
use deno_core::Resource;
use deno_core::ResourceId;
use deno_core::ZeroCopyBuf;

use raw_window_handle::HasRawWindowHandle;
use raw_window_handle::RawWindowHandle;

use winit::dpi::PhysicalPosition;
use winit::dpi::PhysicalSize;
use winit::dpi::Position;
use winit::dpi::Size;
use winit::window::CursorIcon;
use winit::window::Icon;
use winit::window::Window;

use crate::check_unstable;
use crate::event_loop::EventLoopResource;
use crate::hash;

pub struct WindowResource(pub Window);

impl Resource for WindowResource {
  fn name(&self) -> Cow<str> {
    "window".into()
  }
}

unsafe impl HasRawWindowHandle for WindowResource {
  fn raw_window_handle(&self) -> RawWindowHandle {
    self.0.raw_window_handle()
  }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UserAttentionType {
  Critical,
  Informational,
}

impl From<UserAttentionType> for winit::window::UserAttentionType {
  fn from(request_type: UserAttentionType) -> Self {
    match request_type {
      UserAttentionType::Critical => winit::window::UserAttentionType::Critical,
      UserAttentionType::Informational => {
        winit::window::UserAttentionType::Informational
      }
    }
  }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", remote = "CursorIcon")]
pub enum CursorIconDef {
  Default,
  Crosshair,
  Hand,
  Arrow,
  Move,
  Text,
  Wait,
  Help,
  Progress,
  NotAllowed,
  ContextMenu,
  Cell,
  VerticalText,
  Alias,
  Copy,
  NoDrop,
  Grab,
  Grabbing,
  AllScroll,
  ZoomIn,
  ZoomOut,
  EResize,
  NResize,
  NeResize,
  NwResize,
  SResize,
  SeResize,
  SwResize,
  WResize,
  EwResize,
  NsResize,
  NeswResize,
  NwseResize,
  ColResize,
  RowResize,
}

impl From<CursorIconDef> for CursorIcon {
  fn from(def: CursorIconDef) -> CursorIcon {
    match def {
      CursorIconDef::Default => CursorIcon::Default,
      CursorIconDef::Crosshair => CursorIcon::Crosshair,
      CursorIconDef::Hand => CursorIcon::Hand,
      CursorIconDef::Arrow => CursorIcon::Arrow,
      CursorIconDef::Move => CursorIcon::Move,
      CursorIconDef::Text => CursorIcon::Text,
      CursorIconDef::Wait => CursorIcon::Wait,
      CursorIconDef::Help => CursorIcon::Help,
      CursorIconDef::Progress => CursorIcon::Progress,
      CursorIconDef::NotAllowed => CursorIcon::NotAllowed,
      CursorIconDef::ContextMenu => CursorIcon::ContextMenu,
      CursorIconDef::Cell => CursorIcon::Cell,
      CursorIconDef::VerticalText => CursorIcon::VerticalText,
      CursorIconDef::Alias => CursorIcon::Alias,
      CursorIconDef::Copy => CursorIcon::Copy,
      CursorIconDef::NoDrop => CursorIcon::NoDrop,
      CursorIconDef::Grab => CursorIcon::Grab,
      CursorIconDef::Grabbing => CursorIcon::Grabbing,
      CursorIconDef::AllScroll => CursorIcon::AllScroll,
      CursorIconDef::ZoomIn => CursorIcon::ZoomIn,
      CursorIconDef::ZoomOut => CursorIcon::ZoomOut,
      CursorIconDef::EResize => CursorIcon::EResize,
      CursorIconDef::NResize => CursorIcon::NResize,
      CursorIconDef::NeResize => CursorIcon::NeResize,
      CursorIconDef::NwResize => CursorIcon::NwResize,
      CursorIconDef::SResize => CursorIcon::SResize,
      CursorIconDef::SeResize => CursorIcon::SeResize,
      CursorIconDef::SwResize => CursorIcon::SwResize,
      CursorIconDef::WResize => CursorIcon::WResize,
      CursorIconDef::EwResize => CursorIcon::EwResize,
      CursorIconDef::NsResize => CursorIcon::NsResize,
      CursorIconDef::NeswResize => CursorIcon::NeswResize,
      CursorIconDef::NwseResize => CursorIcon::NwseResize,
      CursorIconDef::ColResize => CursorIcon::ColResize,
      CursorIconDef::RowResize => CursorIcon::RowResize,
    }
  }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowSetCursorIconArgs(
  ResourceId,
  #[serde(with = "CursorIconDef")] CursorIcon,
);

pub fn op_windowing_window_new(
  state: &mut deno_core::OpState,
  rid: ResourceId,
  _zero_copy: (),
) -> Result<ResourceId, AnyError> {
  check_unstable(state, "Deno.windowing.Window");
  let event_loop_resource =
    state.resource_table.get::<EventLoopResource>(rid)?;
  let event_loop = event_loop_resource.0.borrow_mut();
  let window_resource = state
    .resource_table
    .add(WindowResource(Window::new(&event_loop)?));

  Ok(window_resource)
}

pub fn op_windowing_window_id(
  state: &mut deno_core::OpState,
  rid: ResourceId,
  _zero_copy: (),
) -> Result<u32, AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(rid)?;
  Ok(hash(window_resource.0.id()))
}

pub fn op_windowing_window_scale_factor(
  state: &mut deno_core::OpState,
  rid: ResourceId,
  _zero_copy: (),
) -> Result<f64, AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(rid)?;
  Ok(window_resource.0.scale_factor())
}

pub fn op_windowing_window_request_redraw(
  state: &mut deno_core::OpState,
  rid: ResourceId,
  _zero_copy: (),
) -> Result<(), AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(rid)?;
  Ok(window_resource.0.request_redraw())
}

pub fn op_windowing_window_inner_position(
  state: &mut deno_core::OpState,
  rid: ResourceId,
  _zero_copy: (),
) -> Result<PhysicalPosition<i32>, AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(rid)?;
  Ok(window_resource.0.inner_position()?)
}

pub fn op_windowing_window_outer_position(
  state: &mut deno_core::OpState,
  rid: ResourceId,
  _zero_copy: (),
) -> Result<PhysicalPosition<i32>, AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(rid)?;
  Ok(window_resource.0.outer_position()?)
}

pub fn op_windowing_window_set_outer_position(
  state: &mut deno_core::OpState,
  (rid, position): (ResourceId, Position),
  _zero_copy: (),
) -> Result<(), AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(rid)?;
  Ok(window_resource.0.set_outer_position(position))
}

pub fn op_windowing_window_inner_size(
  state: &mut deno_core::OpState,
  rid: ResourceId,
  _zero_copy: (),
) -> Result<PhysicalSize<u32>, AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(rid)?;
  Ok(window_resource.0.inner_size())
}

pub fn op_windowing_window_set_inner_size(
  state: &mut deno_core::OpState,
  (rid, size): (ResourceId, Size),
  _zero_copy: (),
) -> Result<(), AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(rid)?;
  Ok(window_resource.0.set_inner_size(size))
}

pub fn op_windowing_window_outer_size(
  state: &mut deno_core::OpState,
  rid: ResourceId,
  _zero_copy: (),
) -> Result<PhysicalSize<u32>, AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(rid)?;
  Ok(window_resource.0.outer_size())
}

pub fn op_windowing_window_set_min_inner_size(
  state: &mut deno_core::OpState,
  (rid, size): (ResourceId, Option<Size>),
  _zero_copy: (),
) -> Result<(), AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(rid)?;
  Ok(window_resource.0.set_min_inner_size(size))
}

pub fn op_windowing_window_set_max_inner_size(
  state: &mut deno_core::OpState,
  (rid, size): (ResourceId, Option<Size>),
  _zero_copy: (),
) -> Result<(), AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(rid)?;
  Ok(window_resource.0.set_max_inner_size(size))
}

pub fn op_windowing_window_set_title(
  state: &mut deno_core::OpState,
  (rid, title): (ResourceId, String),
  _zero_copy: (),
) -> Result<(), AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(rid)?;
  Ok(window_resource.0.set_title(&title))
}

pub fn op_windowing_window_set_visible(
  state: &mut deno_core::OpState,
  (rid, visible): (ResourceId, bool),
  _zero_copy: (),
) -> Result<(), AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(rid)?;
  Ok(window_resource.0.set_visible(visible))
}

pub fn op_windowing_window_set_resizable(
  state: &mut deno_core::OpState,
  (rid, resizable): (ResourceId, bool),
  _zero_copy: (),
) -> Result<(), AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(rid)?;
  Ok(window_resource.0.set_resizable(resizable))
}

pub fn op_windowing_window_set_minimized(
  state: &mut deno_core::OpState,
  (rid, minimized): (ResourceId, bool),
  _zero_copy: (),
) -> Result<(), AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(rid)?;
  Ok(window_resource.0.set_minimized(minimized))
}

pub fn op_windowing_window_set_maximized(
  state: &mut deno_core::OpState,
  (rid, maximized): (ResourceId, bool),
  _zero_copy: (),
) -> Result<(), AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(rid)?;
  Ok(window_resource.0.set_maximized(maximized))
}

pub fn op_windowing_window_is_maximized(
  state: &mut deno_core::OpState,
  rid: ResourceId,
  _zero_copy: (),
) -> Result<bool, AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(rid)?;
  Ok(window_resource.0.is_maximized())
}

pub fn op_windowing_window_set_decorations(
  state: &mut deno_core::OpState,
  (rid, decorations): (ResourceId, bool),
  _zero_copy: (),
) -> Result<(), AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(rid)?;
  Ok(window_resource.0.set_decorations(decorations))
}

pub fn op_windowing_window_set_always_on_top(
  state: &mut deno_core::OpState,
  (rid, always_on_top): (ResourceId, bool),
  _zero_copy: (),
) -> Result<(), AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(rid)?;
  Ok(window_resource.0.set_always_on_top(always_on_top))
}

pub fn op_windowing_window_set_window_icon(
  state: &mut deno_core::OpState,
  (rid, width, height): (ResourceId, u32, u32),
  rgba: ZeroCopyBuf,
) -> Result<(), AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(rid)?;
  Ok(
    window_resource
      .0
      .set_window_icon(Icon::from_rgba(rgba[..].into(), width, height).ok()),
  )
}

pub fn op_windowing_window_set_ime_position(
  state: &mut deno_core::OpState,
  (rid, position): (ResourceId, Position),
  _zero_copy: (),
) -> Result<(), AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(rid)?;
  Ok(window_resource.0.set_ime_position(position))
}

pub fn op_windowing_window_request_user_attention(
  state: &mut deno_core::OpState,
  (rid, request_type): (ResourceId, Option<UserAttentionType>),
  _zero_copy: (),
) -> Result<(), AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(rid)?;
  Ok(window_resource.0.request_user_attention(
    request_type.map(winit::window::UserAttentionType::from),
  ))
}

pub fn op_windowing_window_set_cursor_icon(
  state: &mut deno_core::OpState,
  args: WindowSetCursorIconArgs,
  _zero_copy: (),
) -> Result<(), AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(args.0)?;
  Ok(window_resource.0.set_cursor_icon(args.1))
}

pub fn op_windowing_window_set_cursor_position(
  state: &mut deno_core::OpState,
  (rid, position): (ResourceId, Position),
  _zero_copy: (),
) -> Result<(), AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(rid)?;
  Ok(window_resource.0.set_cursor_position(position)?)
}

pub fn op_windowing_window_set_cursor_grab(
  state: &mut deno_core::OpState,
  (rid, grab): (ResourceId, bool),
  _zero_copy: (),
) -> Result<(), AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(rid)?;
  Ok(window_resource.0.set_cursor_grab(grab)?)
}

pub fn op_windowing_window_set_cursor_visible(
  state: &mut deno_core::OpState,
  (rid, visible): (ResourceId, bool),
  _zero_copy: (),
) -> Result<(), AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(rid)?;
  Ok(window_resource.0.set_cursor_visible(visible))
}

pub fn op_windowing_window_drag_window(
  state: &mut deno_core::OpState,
  rid: ResourceId,
  _zero_copy: (),
) -> Result<(), AnyError> {
  let window_resource = state.resource_table.get::<WindowResource>(rid)?;
  Ok(window_resource.0.drag_window()?)
}
