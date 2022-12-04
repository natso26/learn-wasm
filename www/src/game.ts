import {draw} from "./draw";
import {game_pixels, make_game, step_game} from 'rust-pkg';
import {Config} from "./config";

export type GameState = 'won' | 'lost' | ''

export const startGame = (
    config: Config, canvas: HTMLCanvasElement, getIsInitDraw: () => boolean, getDirection: () => string,
    updateState: (score: number, state: GameState) => void, afterStep: () => void, afterGameOver: () => void,
): { draw: () => void, update: () => void } => {
    const {width, height, frameDurationMs} = config

    let game = make_game(width, height)

    const drawGame = () => {
        draw(config, canvas, game_pixels(game))
        if (!getIsInitDraw()) updateState(game.score, game.state as GameState)
    }

    return {
        draw: drawGame,
        update: () => update(frameDurationMs, drawGame, () => {
                const nextGame = step_game(game, getDirection())

                if (nextGame === undefined) return false;

                game = nextGame

                afterStep()

                return game.state === ''
            },
            () => {
                drawGame()
                afterGameOver()
            }),
    }
}

const update = (frameDurationMs: number, draw: () => void, step: () => boolean, afterEnd: () => void) => {
    draw()

    setTimeout(() => {
        if (step()) requestAnimationFrame(() => update(frameDurationMs, draw, step, afterEnd))
        else afterEnd()
    }, frameDurationMs)
}
