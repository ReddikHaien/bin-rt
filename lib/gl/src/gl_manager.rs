use deno_core::plugin_api::Op;
use deno_core::plugin_api::OpResponse;
use deno_core::plugin_api::Interface;
use deno_core::plugin_api::ZeroCopyBuf;
use gl;



pub fn initialize_plugin(interface: &mut dyn Interface){
    interface.register_op("op_create_buffer", create_buffer);
}

pub fn create_buffer(_interface: &mut dyn Interface, _zero_copy: Option<ZeroCopyBuf>) -> Op{
    let mut index: u32 = 0;
    unsafe{
        gl::CreateBuffers(1,&mut index);
    }
    Op::Sync(OpResponse::Buffer(Box::new(index.to_be_bytes())))
}

pub fn set_buffer_data(_interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op{
    Op::Sync(OpResponse::Buffer(Box::new([0])))
}