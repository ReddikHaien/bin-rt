#[path = "./data.rs"] mod data;

use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::OpResponse;
use deno_core::plugin_api::ZeroCopyBuf;
use futures::future::FutureExt;

static mut DATA: Option<data::RenderData> = None;

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface){
    println!("registrerer metoder");
    interface.register_op("op_initialize", initializeRender);

    }


pub fn initializeRender(interface: & mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op{
    let ginstance = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize glfw");
    unsafe {
        DATA = Some(data::RenderData{
            glfw: ginstance,
            windows: std::collections::BTreeMap::new(),
        });
    }
    
    Op::Sync(OpResponse::Buffer(Box::new([0])))
}

pub fn createNewWindow(interface: & mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op{
    
}


pub fn terminateRender(interface: & mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op{


    Op::Sync(OpResponse::Buffer(Box::new([0])))
}

