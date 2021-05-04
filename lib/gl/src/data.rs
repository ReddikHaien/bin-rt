extern crate glfw;
extern crate gl;

use gl::types::*;

use glfw::{Action, Glfw};

pub struct RenderData{
    pub glfw: glfw::Glfw,
    pub windows: std::collections::BTreeMap<u32,(glfw::Window,std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>)>
}
