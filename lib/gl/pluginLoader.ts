const filename = "gl";
let suffix = ".so";
let prefix = "lib";

if (Deno.build.os === "windows") {
    suffix = ".dll";
    prefix = "";
}
else if (Deno.build.os === "darwin") {
    suffix = ".dylib";
}
const filepath = `./lib/gl/target/debug/${prefix}${filename}${suffix}`;

const plugin = Deno.openPlugin(filepath); 
(Deno as any).core.syncOpsCache();

let ops:{[x: string]: number}  = (Deno as any).core.ops();

export const opRenderInitialize = ops.op_render_initialize;
export const opPushBuffer = ops.op_push_buffer;
export const opPopBuffer = ops.op_pop_buffer;

export const opCreateBuffer = ops.op_create_buffer;
export const opDeleteBuffer = ops.op_delete_buffer;
export const opBindBuffer = ops.op_bind_buffer;
export const opSetBufferDataSize = ops.op_set_buffer_data_size;
export const opSetBufferDataArr = ops.op_set_buffer_data_arr;
export const opSetBufferSubData = ops.op_set_buffer_sub_data;

export const opSetClearColor = ops.op_set_clear_color;
export const opClear = ops.op_clear;

export const opShouldWindowClose = ops.op_should_window_close;
export const opPollEvents = ops.op_poll_events;
export const opSwapWindowBuffers = ops.op_swap_buffers;

export function closePlugin(){
    Deno.close(plugin);
}