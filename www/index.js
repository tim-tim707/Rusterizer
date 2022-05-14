import {
    Scene
} from "wasm-game-of-life";

const canvas = document.getElementById("canvas");
canvas.width = 1600;
canvas.height = 900;

const scene = Scene.new_teapot(canvas);
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
const scene_input = (event) => {
    scene.input(event.key);
}

const play = () => {
    playPauseButton.textContent = "⏸";
    document.addEventListener("keypress", scene_input);
    renderLoop();
};

const pause = () => {
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(animationId);
    document.removeEventListener("keypress", scene_input);
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