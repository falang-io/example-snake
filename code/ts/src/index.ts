import { IDrawRectParams, IDrawCircleParams } from './falang/GameApi';
import { main } from "./falang/main";

enum PressedButton {
  None,
  Left,
  Right,
  Top,
  Bottom,
}

const run = () => {
  let canvas = document.getElementById('canvas') as HTMLCanvasElement;
  if(!canvas) throw new Error();
  let ctx = canvas.getContext('2d')!;
  if(!ctx) throw new Error('');
  ctx.strokeStyle = 'transparent';
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  canvas.width = 800;
  canvas.height = 600;
  
  let pressedButton: PressedButton = PressedButton.None;

  document.addEventListener('keydown', (e) => {
    if ( e.keyCode == 87 || e.keyCode == 38 ) { // W (up) or arrow up
      pressedButton = PressedButton.Top;
    }
    else if ( e.keyCode == 65 || e.keyCode == 37 ) { // A (left) or arrow left
      pressedButton = PressedButton.Left;
    }
    else if ( e.keyCode == 83 || e.keyCode == 40 ) { // S (down) or arrow down
      pressedButton = PressedButton.Bottom;
    }
    else if ( e.keyCode == 68 || e.keyCode == 39 ) { // D (right) or arrow right
      pressedButton = PressedButton.Right;
    }
  });

  (document.getElementById('btn-up') as HTMLButtonElement).addEventListener('click', () => {
    pressedButton = PressedButton.Top;
  });

  (document.getElementById('btn-down') as HTMLButtonElement).addEventListener('click', () => {
    pressedButton = PressedButton.Bottom;
  });

  (document.getElementById('btn-left') as HTMLButtonElement).addEventListener('click', () => {
    pressedButton = PressedButton.Left;
  });

  (document.getElementById('btn-right') as HTMLButtonElement).addEventListener('click', () => {
    pressedButton = PressedButton.Right;
  });

  main({
    config: {
      cellSize: 40,
      foodSize: 15,
      pointRadius: 6,
      gameWidth: 20,
      gameHeight: 15,
    },
    _falangGlobal: {
      apis: {
        GameApi: {
          Application: {
            GetButtonsState: async() => {
              let btn = pressedButton;
              pressedButton = PressedButton.None;
              return {
                bottom: btn == PressedButton.Bottom,
                top: btn == PressedButton.Top,
                left: btn == PressedButton.Left,
                right: btn == PressedButton.Right,
              };
            },
            GetRandom: async ({ min, max }) => {
              return Math.floor(Math.random() * (max - min)) + min;
            },
            Sleep: async () => {
              await new Promise((resolve) => setTimeout(resolve, 150));
            }
          },
          Drawing: {
            DrawRect: async ({ x, y, h, w, color }: IDrawRectParams): Promise<void> => {
              ctx.fillStyle = `rgb(${color.r},${color.g},${color.b})`;
              ctx.fillRect(x, y, w, h);
            },
            DrawCircle: async ({color, r, x, y}: IDrawCircleParams): Promise<void> => {
              ctx.fillStyle = `rgb(${color.r},${color.g},${color.b})`;
              ctx.beginPath();
              ctx.arc(x, y, r, 0, 2 * Math.PI);
              ctx.fill();
            }
          },
        }
      }
    }
  });
}

window.addEventListener('load', function() {
  run();
});



