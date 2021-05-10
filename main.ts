import { GlEnums, initializeRender} from './lib/gl/mod.ts';

const gl = initializeRender({
    heigth: 600,
    width: 800,
    title: "Dette er en test"
});


while(!gl.shouldWindowClose()){
    gl.pollEvents();


    gl.swapWindowBuffers();
}