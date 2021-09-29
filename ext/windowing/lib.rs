// Copyright 2021 the Deno authors. All rights reserved. MIT license.

use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use deno_core::include_js_files;
use deno_core::op_sync;
use deno_core::Extension;
use deno_core::OpState;

mod event;
mod event_loop;
mod window;

pub struct Unstable(pub bool);

pub fn check_unstable(state: &OpState, api_name: &str) {
  let unstable = state.borrow::<Unstable>();

  if !unstable.0 {
    eprintln!(
      "Unstable API '{}'. The --unstable flag must be provided.",
      api_name
    );
    std::process::exit(70);
  }
}

pub fn hash<T: Hash>(t: T) -> u32 {
  let mut s = DefaultHasher::new();
  t.hash(&mut s);
  s.finish() as u32
}

pub fn init(unstable: bool) -> Extension {
  Extension::builder()
    .js(include_js_files!(
      prefix "deno:ext/windowing",
      "00_windowing.js",
    ))
    .ops(vec![
      (
        "op_windowing_event_loop_new",
        op_sync(event_loop::op_windowing_event_loop_new),
      ),
      (
        "op_windowing_event_loop_step",
        op_sync(event_loop::op_windowing_event_loop_step),
      ),
      (
        "op_windowing_window_new",
        op_sync(window::op_windowing_window_new),
      ),
      (
        "op_windowing_window_id",
        op_sync(window::op_windowing_window_id),
      ),
      (
        "op_windowing_window_scale_factor",
        op_sync(window::op_windowing_window_scale_factor),
      ),
      (
        "op_windowing_window_request_redraw",
        op_sync(window::op_windowing_window_request_redraw),
      ),
      (
        "op_windowing_window_inner_position",
        op_sync(window::op_windowing_window_inner_position),
      ),
      (
        "op_windowing_window_outer_position",
        op_sync(window::op_windowing_window_outer_position),
      ),
      (
        "op_windowing_window_set_outer_position",
        op_sync(window::op_windowing_window_set_outer_position),
      ),
      (
        "op_windowing_window_inner_size",
        op_sync(window::op_windowing_window_inner_size),
      ),
      (
        "op_windowing_window_set_inner_size",
        op_sync(window::op_windowing_window_set_inner_size),
      ),
      (
        "op_windowing_window_outer_size",
        op_sync(window::op_windowing_window_outer_size),
      ),
      (
        "op_windowing_window_set_min_inner_size",
        op_sync(window::op_windowing_window_set_min_inner_size),
      ),
      (
        "op_windowing_window_set_max_inner_size",
        op_sync(window::op_windowing_window_set_max_inner_size),
      ),
      (
        "op_windowing_window_set_title",
        op_sync(window::op_windowing_window_set_title),
      ),
      (
        "op_windowing_window_set_visible",
        op_sync(window::op_windowing_window_set_visible),
      ),
      (
        "op_windowing_window_set_resizable",
        op_sync(window::op_windowing_window_set_resizable),
      ),
      (
        "op_windowing_window_set_minimized",
        op_sync(window::op_windowing_window_set_minimized),
      ),
      (
        "op_windowing_window_set_maximized",
        op_sync(window::op_windowing_window_set_maximized),
      ),
      (
        "op_windowing_window_is_maximized",
        op_sync(window::op_windowing_window_is_maximized),
      ),
      (
        "op_windowing_window_set_decorations",
        op_sync(window::op_windowing_window_set_decorations),
      ),
      (
        "op_windowing_window_set_always_on_top",
        op_sync(window::op_windowing_window_set_always_on_top),
      ),
      (
        "op_windowing_window_set_window_icon",
        op_sync(window::op_windowing_window_set_window_icon),
      ),
      (
        "op_windowing_window_set_ime_position",
        op_sync(window::op_windowing_window_set_ime_position),
      ),
      (
        "op_windowing_window_request_user_attention",
        op_sync(window::op_windowing_window_request_user_attention),
      ),
      (
        "op_windowing_window_set_cursor_icon",
        op_sync(window::op_windowing_window_set_cursor_icon),
      ),
      (
        "op_windowing_window_set_cursor_position",
        op_sync(window::op_windowing_window_set_cursor_position),
      ),
      (
        "op_windowing_window_set_cursor_grab",
        op_sync(window::op_windowing_window_set_cursor_grab),
      ),
      (
        "op_windowing_window_set_cursor_visible",
        op_sync(window::op_windowing_window_set_cursor_visible),
      ),
      (
        "op_windowing_window_drag_window",
        op_sync(window::op_windowing_window_drag_window),
      ),
    ])
    .state(move |state| {
      state.put(Unstable(unstable));
      Ok(())
    })
    .build()
}
