import { Scene } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";


const canvas = document.getElementById("smiley_canvas");
console.log(canvas.width, canvas.height);

const scene = Scene.new(canvas);
let animationId = null;

const renderLoop = () => {
    scene.tick();
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