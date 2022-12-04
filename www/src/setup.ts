import {Config} from "./config";
import {GameState, startGame} from "./game";

export const setup = (
    config: Config, canvas: HTMLCanvasElement, startButton: HTMLButtonElement,
    scoreElement: HTMLElement, statusElement: HTMLElement,
): void => {
    const {width, height, pixelSize} = config

    canvas.width = width * pixelSize + 2
    canvas.height = height * pixelSize + 2

    let direction = ''

    document.addEventListener('keydown', (event) => {
        const code = event.code

        if (code === 'ArrowUp') direction = 'up'
        else if (code === 'ArrowDown') direction = 'down'
        else if (code === 'ArrowLeft') direction = 'left'
        else if (code === 'ArrowRight') direction = 'right'
    })

    let isInitDraw = true
    let draw: () => void = () => {
    }
    let update: () => void = () => {
    }

    const start = () => startGame(
        config, canvas, () => isInitDraw, () => direction,
        (score: number, state: GameState) => {
            scoreElement.textContent = score.toString()
            statusElement.textContent = state === 'won' ? 'You won.' : state === 'lost' ? 'You lost.' : ''
        },
        () => direction = '',
        () => {
            startButton.disabled = false
            const {draw: new_draw, update: new_update} = start()
            draw = new_draw
            update = new_update
        },
    )

    const {draw: init_draw, update: init_update} = start()
    draw = init_draw
    update = init_update

    draw()

    startButton.addEventListener('click', () => {
        startButton.disabled = true

        isInitDraw = false
        update()
    })
}
