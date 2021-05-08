mod data;
mod util;
mod gl_manager;
mod glfw_manager;
mod buffer_manager;


use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::OpResponse;
use deno_core::plugin_api::ZeroCopyBuf;


#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    println!("registrerer metoder");
    gl_manager::initialize_plugin(interface);
    buffer_manager::initialize(interface);
    glfw_manager::initialize_plugin(interface);
    interface.register_op("op_render_initialize", initialize_render);
}

pub fn initialize_render(_interface: &mut dyn Interface, _zero_copy: Option<ZeroCopyBuf>) -> Op {

    glfw_manager::initialize_glfw(String::from("Test"), 800, 600);
    
    Op::Sync(OpResponse::Buffer(Box::new([0])))
}





