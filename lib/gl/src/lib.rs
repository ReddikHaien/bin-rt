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

pub fn initialize_render(_interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op {

    let name_buf = buffer_manager::get_buffer().expect("expected buffer for name, recieved nothing");
    let slice = &zero_copy.unwrap()[..];
    let width = util::slice_to_int(slice, 0);
    let height = util::slice_to_int(slice, 4);

    glfw_manager::initialize_glfw(String::from_utf8(name_buf.to_vec()).expect("Failed to convert slice to string"), width, height);
    
    Op::Sync(OpResponse::Buffer(Box::new([0])))
}