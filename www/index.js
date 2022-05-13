import {
    Scene
} from "wasm-game-of-life";
import {
    memory
} from "wasm-game-of-life/wasm_game_of_life_bg";


const canvas = document.getElementById("canvas");
canvas.width = 1600;
canvas.height = 900;

const scene = Scene.new(canvas);
let animationId = null;
var tick_nb = 0

const renderLoop = () => {
    scene.tick(tick_nb);
    tick_nb += 0.01;
    animationId = requestAnimationFrame(renderLoop);
};

const isPaused = () => {
    return animationId === null;
};

const playPauseButton = document.getElementById("play-pause");

const play = () => {
    playPauseButton.textContent = "⏸";
    renderLoop();
};

const pause = () => {
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(animationId);
    animationId = null;
};

playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});

play();