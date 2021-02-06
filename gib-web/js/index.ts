import init, {index} from "../static/wasm/gib_web";

// @ts-ignore
(async () => {
    await init("wasm/gib_web_bg.wasm");
    index();
})();