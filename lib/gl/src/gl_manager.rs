use deno_core::plugin_api::Op;
use deno_core::plugin_api::OpResponse;
use deno_core::plugin_api::Interface;
use deno_core::plugin_api::ZeroCopyBuf;
use gl;

use crate::util;
use crate::buffer_manager;

pub fn initialize_plugin(interface: &mut dyn Interface){
    interface.register_op("op_create_buffer", create_buffer);
    interface.register_op("op_delete_buffer", delete_buffer);
    interface.register_op("op_bind_buffer", bind_buffer);
    interface.register_op("op_set_buffer_data_size", set_buffer_data_size);
    interface.register_op("op_set_buffer_data_arr", set_buffer_data_arr);
    interface.register_op("op_set_buffer_sub_data", set_buffer_sub_data);
    
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

pub fn delete_buffer(_interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op{
    let mut index = util::slice_to_int(&zero_copy.unwrap(),0);
    unsafe{
        gl::DeleteBuffers(1,&mut index);
    }
    Op::Sync(OpResponse::Buffer(Box::new([0])))
}

pub fn bind_buffer(_interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op{
    let slice = &zero_copy.unwrap();
    let target = util::slice_to_int(slice,0); 
    let index = util::slice_to_int(slice,4);
    unsafe{
        gl::BindBuffer(target,index);
    }
    Op::Sync(OpResponse::Buffer(Box::new([0])))
}

fn set_buffer_data_size(_interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op{
    let slice = &zero_copy.unwrap()[..];
    let target = util::slice_to_int(slice, 0);
    let size = util::slice_to_int(slice, 4) as isize;
    let usage = util::slice_to_int(slice, 8);

    unsafe{
        gl::BufferData(target,size,std::ptr::null(),usage);
    }

    Op::Sync(OpResponse::Buffer(Box::new([0])))
}

fn set_buffer_data_arr(_interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op{
    let slice = &zero_copy.unwrap()[..];
    let target = util::slice_to_int(slice, 0);
    let usage = util::slice_to_int(slice, 4);
    let buffer = buffer_manager::get_buffer().unwrap();
    let size = buffer.len() as isize;

    unsafe{
        gl::BufferData(target,size,buffer.as_ptr() as *const std::ffi::c_void ,usage);
    }

    Op::Sync(OpResponse::Buffer(Box::new([0])))
}

fn set_buffer_sub_data(_interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op{
    let slice = &zero_copy.unwrap();
    let target = util::slice_to_int(slice, 0);
    let offset = util::slice_to_int(slice, 4) as isize;
    let source_offset = util::slice_to_int(slice, 8) as usize;
    let source = buffer_manager::get_buffer().unwrap();
    
    if source_offset == 0{
        unsafe{
            gl::BufferSubData(target,offset,source.len() as isize, source.as_ptr() as *const std::ffi::c_void);
        }
    }
    else{
        let expected_size = source.len() - source_offset;
        let source_slice = &source[source_offset..expected_size];
        unsafe{
            gl::BufferSubData(target,offset,expected_size as isize,source_slice.as_ptr() as *const std::ffi::c_void);
        }
    }

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