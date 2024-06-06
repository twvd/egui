//! Containers are pieces of the UI which wraps other pieces of UI. Examples: [`Window`], [`ScrollArea`], [`Resize`], [`SidePanel`], etc.
//!
//! For instance, a [`Frame`] adds a frame and background to some contained UI.

pub(crate) mod area;
pub mod collapsing_header;
mod combo_box;
pub(crate) mod frame;
pub mod group;
pub mod panel;
pub mod popup;
pub(crate) mod resize;
pub mod scroll_area;
pub(crate) mod window;

pub use {
    area::{Area, AreaState},
    collapsing_header::{CollapsingHeader, CollapsingResponse},
    combo_box::*,
    frame::Frame,
    group::Group,
    panel::{CentralPanel, SidePanel, TopBottomPanel},
    popup::*,
    resize::Resize,
    scroll_area::ScrollArea,
    window::Window,
};
