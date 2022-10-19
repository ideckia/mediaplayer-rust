const { MediaPlayer } = require('.');

const mp = new MediaPlayer();
const id = mp.play('/your/sound_file/path', false, () => console.log('Sound ended'));

let paused = false;
const interval = setInterval(() => {
    if (paused)
        mp.resume(id);
    else
        mp.pause(id);

    paused = !paused;
}, 2000);

setTimeout(() => {
    mp.stop(id);
    clearInterval(interval);
}, 8000);