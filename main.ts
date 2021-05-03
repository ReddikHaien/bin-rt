import { Render, WindowEvents, Action } from './lib/gl/mod.ts';


Render.initialize();
const window = Render.createWindow();
Render.makeWindowCurrent(window);

Render.setClearColor(0,0,0,1);

console.log("ferdig med init");

while(!Render.shouldClose(window)){
    Render.clear(Render.COLOR_BUFFER_BIT);
    Render.pollWindow(window);
    Render.swapWindowBuffer(window);
}

Render.cleanUp();
console.log("Dette virket!!");



