use deno_core::plugin_api::Op;
use deno_core::plugin_api::OpResponse;
use deno_core::plugin_api::Interface;
use deno_core::plugin_api::ZeroCopyBuf;
use gl;

use crate::util;

pub fn initialize_plugin(interface: &mut dyn Interface){
    interface.register_op("op_create_buffer", create_buffer);
    interface.register_op("op_set_clear_color", set_clear_color);
    interface.register_op("op_clear", clear);
}

pub fn create_buffer(_interface: &mut dyn Interface, _zero_copy: Option<ZeroCopyBuf>) -> Op{
    let mut index: u32 = 0;
    unsafe{
        gl::CreateBuffers(1,&mut index);
    }
    Op::Sync(OpResponse::Buffer(Box::new(index.to_be_bytes())))
}

fn set_buffer_data_size(_interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op{
    let slice = &zero_copy.unwrap()[..];
    let target = util::slice_to_int(slice, 0);
    Op::Sync(OpResponse::Buffer(Box::new([0])))
}

fn set_buffer_data_arr(_interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op{
    Op::Sync(OpResponse::Buffer(Box::new([0])))
}


fn set_clear_color(_interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op{
    let slice = &zero_copy.unwrap()[..];
    let r = util::slice_to_float(slice, 0);
    let g = util::slice_to_float(slice, 4);
    let b = util::slice_to_float(slice, 8);
    let a = util::slice_to_float(slice, 12);

    unsafe{
        gl::ClearColor(r,g,b,a);
    }

    Op::Sync(OpResponse::Buffer(Box::new([0])))
}

fn clear(_interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op{
    let slice = &zero_copy.unwrap()[..];
    let mask = util::slice_to_int(slice, 0);
    
    unsafe{
        gl::Clear(mask);
    }

    Op::Sync(OpResponse::Buffer(Box::new([0])))
}