import init from 'rust-pkg';
import {config} from "./config";
import {setup} from "./setup";

(async () => {
    await init()

    const canvas = document.getElementById('canvas') as HTMLCanvasElement
    const startButton = document.getElementById('start') as HTMLButtonElement
    const scoreElement = document.getElementById('score') as HTMLElement
    const statusElement = document.getElementById('status') as HTMLElement

    setup(config, canvas, startButton, scoreElement, statusElement)
})()
