import { Render, WindowEvents, Action } from './lib/gl/mod.ts';


Render.initialize();
const window = Render.createWindow();
Render.makeWindowCurrent(window);

Render.addListener(WindowEvents.KeyEvent,(key,scancode,action,modifier) =>{
    console.log(key+"("+scancode+") " + Action[action]);
});

console.log("ferdig med init");

while(!Render.shouldClose(window) ) {
    Render.pollWindow(window);
}

Render.cleanUp();
console.log("Dette virket!!");