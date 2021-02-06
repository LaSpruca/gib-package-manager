import init, {index} from "../static/wasm/gib_web";
import "../style/main.scss";

// @ts-ignore
(async () => {
    await init("wasm/gib_web_bg.wasm");
    index();
})();