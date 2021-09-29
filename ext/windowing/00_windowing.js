// Copyright 2018-2021 the Deno authors. All rights reserved. MIT license.
"use strict";

((window) => {
  const core = window.Deno.core;

  class EventLoop {
    #rid;

    constructor() {
      this.#rid = core.opSync("op_windowing_event_loop_new");
    }

    get rid() {
      return this.#rid;
    }

    step() {
      return core.opSync("op_windowing_event_loop_step", this.#rid);
    }

    close() {
      core.close(this.#rid);
    }
  }

  class Window {
    #rid;

    constructor(eventLoop) {
      this.#rid = core.opSync("op_windowing_window_new", eventLoop.rid);
    }

    get id() {
      return core.opSync("op_windowing_window_id", this.#rid);
    }

    scaleFactor() {
      return core.opSync("op_windowing_window_scale_factor", this.#rid);
    }

    requestRedraw() {
      core.opSync("op_windowing_window_request_redraw", this.#rid);
    }

    innerPosition() {
      return core.opSync("op_windowing_window_inner_position", this.#rid);
    }

    outerPosition() {
      return core.opSync("op_windowing_window_outer_position", this.#rid);
    }

    setOuterPosition(position) {
      core.opSync("op_windowing_window_set_outer_position", [
        this.#rid,
        position,
      ]);
    }

    innerSize() {
      return core.opSync("op_windowing_window_inner_size", this.#rid);
    }

    setInnerSize(size) {
      core.opSync("op_windowing_window_set_inner_size", [this.#rid, size]);
    }

    outerSize() {
      return core.opSync("op_windowing_window_outer_size", this.#rid);
    }

    setMinInnerSize(size) {
      core.opSync("op_windowing_window_set_min_inner_size", [this.#rid, size]);
    }

    setMaxInnerSize(size) {
      core.opSync("op_windowing_window_set_max_inner_size", [this.#rid, size]);
    }

    setTitle(title) {
      core.opSync("op_windowing_window_set_title", [this.#rid, title]);
    }

    setVisible(visible) {
      core.opSync("op_windowing_window_set_visible", [this.#rid, visible]);
    }

    setResizable(resizable) {
      core.opSync("op_windowing_window_set_resizable", [this.#rid, resizable]);
    }

    setMinimized(minimized) {
      core.opSync("op_windowing_window_set_minimized", [this.#rid, minimized]);
    }

    setMaximized(maximized) {
      core.opSync("op_windowing_window_set_maximized", [this.#rid, maximized]);
    }

    isMaximized() {
      return core.opSync("op_windowing_window_is_maximized", this.#rid);
    }

    setDecorations(decorations) {
      core.opSync("op_windowing_window_set_decorations", [
        this.#rid,
        decorations,
      ]);
    }

    setAlwaysOnTop(alwaysOnTop) {
      core.opSync("op_windowing_window_set_always_on_top", [
        this.#rid,
        alwaysOnTop,
      ]);
    }

    setWindowIcon(
      rgba,
      width,
      height,
    ) {
      core.opSync("op_windowing_window_set_window_icon", [
        this.#rid,
        width,
        height,
      ], rgba);
    }

    setImePosition(position) {
      core.opSync("op_windowing_window_set_ime_position", [
        this.#rid,
        position,
      ]);
    }

    requestUserAttention(requestType) {
      core.opSync("op_windowing_window_request_user_attention", [
        this.#rid,
        requestType,
      ]);
    }

    setCursorIcon(cursor) {
      core.opSync("op_windowing_window_set_cursor_icon", [
        this.#rid,
        cursor,
      ]);
    }

    setCursorPosition(position) {
      core.opSync("op_windowing_window_set_cursor_position", [
        this.#rid,
        position,
      ]);
    }

    setCursorGrab(grab) {
      core.opSync("op_windowing_window_set_cursor_grab", [this.#rid, grab]);
    }

    setCursorVisible(visible) {
      core.opSync("op_windowing_window_set_cursor_visible", [
        this.#rid,
        visible,
      ]);
    }

    dragWindow() {
      core.opSync("op_windowing_window_set_cursor_visible", this.#rid);
    }

    close() {
      core.close(this.#rid);
    }
  }

  window.__bootstrap.windowing = { EventLoop, Window };
})(this);
