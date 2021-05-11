import { GlEnums, initializeRender} from './lib/gl/mod.ts';

const gl = initializeRender({
    heigth: 600,
    width: 800,
    title: "Dette er en test"
});

gl.clearColor(0.9,1,0.8,1);

while(!gl.shouldWindowClose()){
    gl.pollEvents();

    gl.clear(GlEnums.COLOR_BUFFER_BIT);

    gl.swapWindowBuffers();
}