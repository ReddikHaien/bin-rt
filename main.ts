import { Render, WindowEvents, Action } from './lib/gl/mod.ts';


Render.initialize();
const window = Render.createWindow();
window.makeCurrent();

Render.setClearColor(0,0,0,1);

console.log("ferdig med init");

while(!window.shouldClose()){
    Render.clear(Render.COLOR_BUFFER_BIT);
    window.poll();
    window.swapBuffers();
}

Render.cleanUp();
console.log("Dette virket!!");



