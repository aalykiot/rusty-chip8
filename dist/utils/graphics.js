const render = (canvas, chip8, memory) => {
  const WIDTH = 64;
  const HEIGHT = 32;
  const display_ptr = chip8.display.get_ptr();
  const display = new Uint8Array(memory.buffer, display_ptr, WIDTH * HEIGHT);
  const ctx = canvas.getContext("2d");
  ctx.fillStyle = "#1a202c";
  ctx.fillRect(0, 0, canvas.width, canvas.height);
  for (let x = 0; x < WIDTH; x++) {
    for (let y = 0; y < HEIGHT; y++) {
      let index = x + WIDTH * y;
      if (display[index] === 1) {
        ctx.fillStyle = "white";
        ctx.fillRect(x * 10, y * 10, 10, 10);
      }
    }
  }
};
export {render};
