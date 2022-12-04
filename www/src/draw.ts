import {Config} from "./config";

export type Pixel = {
    x: number,
    y: number,
    kind: 'snake' | 'food' | 'empty',
    snakePosition: number | undefined,
}

export const draw = (config: Config, canvas: HTMLCanvasElement, pixels: Pixel[]) => {
    const ctx = canvas.getContext('2d')
    ctx.clearRect(0, 0, canvas.width, canvas.height)

    drawPixels(config, ctx, pixels)
    drawGrid(config, ctx)
}

const drawPixels = (config: Config, ctx: CanvasRenderingContext2D, pixels: Pixel[]) => {
    const {pixelSize} = config

    const maxSnakePosition = pixels.reduce((max, pixel) => {
        if (pixel.kind === 'snake' && pixel.snakePosition !== undefined) {
            return Math.max(max, pixel.snakePosition)
        } else {
            return max
        }
    }, 0)

    for (const pixel of pixels) {
        const {x, y, kind, snakePosition} = pixel

        if (kind === 'snake') {
            if (snakePosition === 0) ctx.fillStyle = 'blue'
            else {
                const relativeSnakePosition = (snakePosition - 1) / (maxSnakePosition - 1 || 1)
                ctx.fillStyle = `rgb(${75 * (1 - relativeSnakePosition)}, 0, ${130 * (1 - relativeSnakePosition)})`
            }
        } else if (kind === 'food') ctx.fillStyle = 'red'
        else ctx.fillStyle = 'transparent'

        ctx.fillRect(x * pixelSize + 1, y * pixelSize + 1, pixelSize, pixelSize)
    }
}

const drawGrid = (config: Config, ctx: CanvasRenderingContext2D) => {
    const {width, height, pixelSize} = config

    ctx.beginPath()

    for (let x = 0; x < width + 1; x++) {
        ctx.moveTo(x * pixelSize + 1, 1)
        ctx.lineTo(x * pixelSize + 1, height * pixelSize + 1)
    }

    for (let y = 0; y < height + 1; y++) {
        ctx.moveTo(1, y * pixelSize + 1)
        ctx.lineTo(width * pixelSize + 1, y * pixelSize + 1)
    }

    ctx.stroke()
}
