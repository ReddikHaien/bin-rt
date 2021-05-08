use deno_core::ZeroCopyBuf;
use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::OpResponse;

struct BufferStack{
    stack: Vec<ZeroCopyBuf>,
}

static mut STACK: Option<BufferStack> = None;

pub fn initialize(interface: & mut dyn Interface){
    interface.register_op("op_push_buffer", push_buffer);
    interface.register_op("op_pop_buffer", pop_buffer);

    unsafe{
        STACK = Some(BufferStack{
            stack: Vec::new()
        });
    }
}

pub fn push_buffer(_interface: & mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op{
    match zero_copy{
        Some(d) => {
            unsafe {
                match STACK{
                    Some(ref mut stack) => {
                        stack.stack.push(d);
                    },
                    None => panic!("plugin not initialized!")
                }
            }
        }
        None => panic!("expected buffer recieved nothing")
    }
    Op::Sync(OpResponse::Buffer(Box::new([0])))
}

pub fn pop_buffer(_interface: & mut dyn Interface, _zero_copy: Option<ZeroCopyBuf>) -> Op{
    unsafe{
        match STACK{
            Some(ref mut d) =>{
                d.stack.pop();
            },
            None => ()
        }
    }

    Op::Sync(OpResponse::Buffer(Box::new([0])))
}