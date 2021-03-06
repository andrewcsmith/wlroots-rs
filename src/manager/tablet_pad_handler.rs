//! Handler for tablet pads

use libc;
use wlroots_sys::wlr_input_device;
use wayland_sys::server::WAYLAND_SERVER_HANDLE;

use {TabletPad, TabletPadHandle};
use compositor::{compositor_handle, CompositorHandle};
use events::tablet_pad_events::{ButtonEvent, RingEvent, StripEvent};

pub trait TabletPadHandler {
    /// Callback that is triggered when a button is pressed on the tablet pad.
    fn on_button(&mut self, CompositorHandle, TabletPadHandle, &ButtonEvent) {}

    /// Callback that is triggered when the touch strip is used.
    fn on_strip(&mut self, CompositorHandle, TabletPadHandle, &StripEvent) {}

    /// Callback that is triggered when the ring is touched.
    fn on_ring(&mut self, CompositorHandle, TabletPadHandle, &RingEvent) {}

    /// Callback that is triggered when the pad device is destroyed.
    fn destroyed(&mut self, CompositorHandle, TabletPadHandle) {}
}

wayland_listener!(TabletPadWrapper, (TabletPad, Box<TabletPadHandler>), [
    on_destroy_listener => on_destroy_notify: |this: &mut TabletPadWrapper, data: *mut libc::c_void,|
    unsafe {
        let input_device_ptr = data as *mut wlr_input_device;
        {
            let (ref mut pad, ref mut tablet_pad_handler) = this.data;
            let compositor = match compositor_handle() {
                Some(handle) => handle,
                None => return
            };
            tablet_pad_handler.destroyed(compositor, pad.weak_reference());
        }
        ffi_dispatch!(WAYLAND_SERVER_HANDLE,
                      wl_list_remove,
                      &mut (*this.on_destroy_listener()).link as *mut _ as _);
        ffi_dispatch!(WAYLAND_SERVER_HANDLE,
                      wl_list_remove,
                      &mut (*this.button_listener()).link as *mut _ as _);
        ffi_dispatch!(WAYLAND_SERVER_HANDLE,
                      wl_list_remove,
                      &mut (*this.ring_listener()).link as *mut _ as _);
        ffi_dispatch!(WAYLAND_SERVER_HANDLE,
                      wl_list_remove,
                      &mut (*this.strip_listener()).link as *mut _ as _);
        Box::from_raw((*input_device_ptr).data as *mut TabletPadWrapper);
    };
    button_listener => button_notify: |this: &mut TabletPadWrapper, data: *mut libc::c_void,|
    unsafe {
        let (ref pad, ref mut handler) = this.data;
        let event = ButtonEvent::from_ptr(data as *mut _);
        let compositor = match compositor_handle() {
            Some(handle) => handle,
            None => return
        };

        handler.on_button(compositor,
                          pad.weak_reference(),
                          &event);
    };
    strip_listener => strip_notify: |this: &mut TabletPadWrapper, data: *mut libc::c_void,|
    unsafe {
        let (ref pad, ref mut handler) = this.data;
        let event = StripEvent::from_ptr(data as *mut _);
        let compositor = match compositor_handle() {
            Some(handle) => handle,
            None => return
        };

        handler.on_strip(compositor,
                         pad.weak_reference(),
                         &event);
    };
    ring_listener => ring_notify: |this: &mut TabletPadWrapper, data: *mut libc::c_void,|
    unsafe {
        let (ref pad, ref mut handler) = this.data;
        let event = RingEvent::from_ptr(data as *mut _);
        let compositor = match compositor_handle() {
            Some(handle) => handle,
            None => return
        };

        handler.on_ring(compositor,
                        pad.weak_reference(),
                        &event);
    };
]);
