import { Render, WindowEvents, Action, GlEnums, GlBuffer} from './lib/gl/mod.ts';

Render.initialize();
const window = Render.createWindow();
window.makeCurrent();

let buffer = GlBuffer.createArrayBuffer();
buffer.setData(new Float32Array([1,2,3]),GlEnums.STATIC_DRAW);

console.log("ferdig med init");

while(!window.shouldClose()){
    Render.clear(GlEnums.COLOR_BUFFER_BIT);
    window.poll();
    window.swapBuffers();
}

Render.cleanUp();
console.log("Dette virket!!");


