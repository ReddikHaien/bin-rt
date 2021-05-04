import { Render, WindowEvents, Action, GlEnums, GlBuffer, createSharedBuffer, printSharedBuffer } from './lib/gl/mod.ts';


Render.initialize();
const window = Render.createWindow();
window.makeCurrent();

Render.setClearColor(0,0,0,1);

let buffer = GlBuffer.createArrayBuffer();
buffer.setData(new Float32Array([1,2,3]),GlEnums.STATIC_DRAW);

console.log("ferdig med init");

let shared = new Uint8Array(4);
createSharedBuffer(shared);
shared[0] = 1;
shared[1] = 2;
shared[2] = 4;
shared[3] = 8; 

printSharedBuffer();

while(!window.shouldClose()){
    Render.clear(GlEnums.COLOR_BUFFER_BIT);
    window.poll();
    window.swapBuffers();
}

Render.cleanUp();
console.log("Dette virket!!");



