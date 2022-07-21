import * as Comlink from "comlink";
Comlink.expose({
    world:undefined,
    range:undefined,
    y:undefined,
    init(size,lineRange,{r,g,b}){
        return import("../pkg").then((wasm)=>{
            wasm.start();
            this.world = new wasm.World(size, r, g, b);
            this.range = lineRange;
            this.y = lineRange.start;
            console.log(this.world)
        });
    },
    renderNext(){
        if (this.y>=this.range.end){
            return false;
        }

        const data=this.world.render(this.y);
        const retVal={y:this.y++ ,data};

        return retVal;
    }
})
