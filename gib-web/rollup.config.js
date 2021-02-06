import typescript from "@rollup/plugin-typescript";
import scss from "rollup-plugin-scss";

export default [{
    input: 'js/index.ts',
    output: {
        file: 'static/index.js',
        format: 'cjs'
    },
    plugins: [
        typescript(),
        scss()
    ]
}];