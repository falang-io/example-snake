"use strict";
(() => {
  // src/falang/getColors.ts
  async function getColors(_params) {
    const _falangGlobal = _params._falangGlobal;
    let returnValue = { foodColor: { r: 0, g: 0, b: 0 }, cellColor: { r: 0, g: 0, b: 0 }, pointColor: { r: 0, g: 0, b: 0 }, backgroundColor: { r: 0, g: 0, b: 0 } };
    returnValue.foodColor.r = 255;
    returnValue.cellColor.g = 255;
    returnValue.pointColor.r = 50;
    returnValue.pointColor.g = 50;
    returnValue.pointColor.b = 50;
    return returnValue;
  }

  // src/falang/getRandomPoint.ts
  async function getRandomPoint(_params) {
    const _falangGlobal = _params._falangGlobal;
    let state = JSON.parse(JSON.stringify(_params.state));
    let returnValue = { x: 0, y: 0 };
    let x = await _falangGlobal.apis.GameApi.Application.GetRandom({
      min: 0,
      max: state.config.gameWidth
    });
    let y = await _falangGlobal.apis.GameApi.Application.GetRandom({
      min: 0,
      max: state.config.gameHeight
    });
    returnValue.x = x;
    returnValue.y = y;
    return returnValue;
  }

  // src/falang/getNewFoodPoint.ts
  async function getNewFoodPoint(_params) {
    const _falangGlobal = _params._falangGlobal;
    let state = JSON.parse(JSON.stringify(_params.state));
    let returnValue = { x: 0, y: 0 };
    let _continue_level = 0;
    do {
      let newFoodPoint = await getRandomPoint({
        _falangGlobal,
        state
      });
      for (const bodyItem of state.snake.body) {
        if (newFoodPoint.x == bodyItem.x && newFoodPoint.y == bodyItem.y) {
          _continue_level = 1;
          break;
        } else {
        }
      }
      if (_continue_level > 1) {
        _continue_level--;
        break;
      }
      if (_continue_level == 1) {
        _continue_level--;
        continue;
      }
      returnValue.x = newFoodPoint.x;
      returnValue.y = newFoodPoint.y;
    } while (false);
    return returnValue;
  }

  // src/falang/DrawPoints.ts
  async function DrawPoints(_params) {
    const _falangGlobal = _params._falangGlobal;
    let state = JSON.parse(JSON.stringify(_params.state));
    const _x_from = 0;
    const _x_to = state.config.gameWidth;
    for (let x = _x_from; x < _x_to; x++) {
      const _y_from = 0;
      const _y_to = state.config.gameHeight;
      for (let y = _y_from; y < _y_to; y++) {
        await _falangGlobal.apis.GameApi.Drawing.DrawCircle({
          x: x * state.config.cellSize + state.config.cellSize / 2,
          y: y * state.config.cellSize + state.config.cellSize / 2,
          r: state.config.pointRadius,
          color: state.colors.pointColor
        });
      }
    }
  }

  // src/falang/clearScreen.ts
  async function clearScreen(_params) {
    const _falangGlobal = _params._falangGlobal;
    let state = JSON.parse(JSON.stringify(_params.state));
    await _falangGlobal.apis.GameApi.Drawing.DrawRect({
      x: 0,
      y: 0,
      w: state.config.cellSize * state.config.gameWidth,
      h: state.config.cellSize * state.config.gameHeight,
      color: state.colors.backgroundColor
    });
    await DrawPoints({
      _falangGlobal,
      state
    });
  }

  // src/falang/DrawFood.ts
  async function DrawFood(_params) {
    const _falangGlobal = _params._falangGlobal;
    let state = JSON.parse(JSON.stringify(_params.state));
    await _falangGlobal.apis.GameApi.Drawing.DrawCircle({
      x: state.food.x * state.config.cellSize + 1 / 2 + state.config.cellSize / 2,
      y: state.food.y * state.config.cellSize + state.config.cellSize / 2,
      r: state.config.foodSize,
      color: state.colors.foodColor
    });
  }

  // src/falang/drawSnakeSquare.ts
  async function drawSnakeSquare(_params) {
    const _falangGlobal = _params._falangGlobal;
    let point = JSON.parse(JSON.stringify(_params.point));
    let color = JSON.parse(JSON.stringify(_params.color));
    let config = JSON.parse(JSON.stringify(_params.config));
    await _falangGlobal.apis.GameApi.Drawing.DrawRect({
      x: point.x * config.cellSize,
      y: point.y * config.cellSize,
      w: config.cellSize - 2,
      h: config.cellSize - 2,
      color
    });
  }

  // src/falang/getNextPoint.ts
  async function getNextPoint(_params) {
    const _falangGlobal = _params._falangGlobal;
    let state = JSON.parse(JSON.stringify(_params.state));
    let returnValue = { x: 0, y: 0 };
    returnValue.x = state.snake.x + state.snake.dirX;
    returnValue.y = state.snake.y + state.snake.dirY;
    if (returnValue.x < 0) {
      returnValue.x = state.config.gameWidth - 1;
    } else {
    }
    if (returnValue.x >= state.config.gameWidth) {
      returnValue.x = 0;
    } else {
    }
    if (returnValue.y < 0) {
      returnValue.y = state.config.gameHeight - 1;
    } else {
    }
    if (returnValue.y >= state.config.gameHeight) {
      returnValue.y = 0;
    } else {
    }
    return returnValue;
  }

  // src/falang/isGameOver.ts
  async function isGameOver(_params) {
    const _falangGlobal = _params._falangGlobal;
    let state = JSON.parse(JSON.stringify(_params.state));
    let returnValue = false;
    if (state.snake.dirX == 0 && state.snake.dirY == 0) {
      return returnValue;
    } else {
      for (const item of state.snake.body) {
        if (item.x == state.snake.x && item.y == state.snake.y) {
          returnValue = true;
          return returnValue;
        } else {
        }
      }
    }
    return returnValue;
  }

  // src/falang/main.ts
  async function main(_params) {
    const _falangGlobal = _params._falangGlobal;
    let config = JSON.parse(JSON.stringify(_params.config));
    let state = { colors: { foodColor: { r: 0, g: 0, b: 0 }, cellColor: { r: 0, g: 0, b: 0 }, pointColor: { r: 0, g: 0, b: 0 }, backgroundColor: { r: 0, g: 0, b: 0 } }, snake: { x: 0, y: 0, dirX: 0, dirY: 0, body: [] }, food: { x: 0, y: 0 }, config: { cellSize: 0, foodSize: 0, pointRadius: 0, gameWidth: 0, gameHeight: 0 } };
    state.config = config;
    let newColors = await getColors({
      _falangGlobal
    });
    state.colors = newColors;
    let snakePoint = await getRandomPoint({
      _falangGlobal,
      state
    });
    state.snake.x = snakePoint.x;
    state.snake.y = snakePoint.y;
    let foodPoint = await getNewFoodPoint({
      _falangGlobal,
      state
    });
    state.food = foodPoint;
    state.snake.body.push(snakePoint);
    await clearScreen({
      _falangGlobal,
      state
    });
    await DrawFood({
      _falangGlobal,
      state
    });
    await drawSnakeSquare({
      _falangGlobal,
      point: snakePoint,
      color: state.colors.cellColor,
      config: state.config
    });
    do {
      await _falangGlobal.apis.GameApi.Application.Sleep({});
      let buttons = await _falangGlobal.apis.GameApi.Application.GetButtonsState({});
      if (buttons.left) {
        if (state.snake.dirX == 1) {
        } else {
          state.snake.dirX = -1;
          state.snake.dirY = 0;
        }
      } else {
        if (buttons.right) {
          if (state.snake.dirX == -1) {
          } else {
            state.snake.dirX = 1;
            state.snake.dirY = 0;
          }
        } else {
          if (buttons.top) {
            if (state.snake.dirY == 1) {
            } else {
              state.snake.dirX = 0;
              state.snake.dirY = -1;
            }
          } else {
            if (buttons.bottom) {
              if (state.snake.dirY == -1) {
              } else {
                state.snake.dirX = 0;
                state.snake.dirY = 1;
              }
            } else {
            }
          }
        }
      }
      if (state.snake.dirX == 0 && state.snake.dirY == 0) {
        continue;
      } else {
      }
      let nextPoint = await getNextPoint({
        _falangGlobal,
        state
      });
      state.snake.x = nextPoint.x;
      state.snake.y = nextPoint.y;
      let isOver = await isGameOver({
        _falangGlobal,
        state
      });
      if (isOver) {
        state.snake.dirX = 0;
        state.snake.dirY = 0;
        let newSnakePoint = await getRandomPoint({
          _falangGlobal,
          state
        });
        state.snake.body = [newSnakePoint];
        let newFoodPoint = await getNewFoodPoint({
          _falangGlobal,
          state
        });
        state.food = newFoodPoint;
        await _falangGlobal.apis.GameApi.Drawing.DrawRect({
          x: 0,
          y: 0,
          w: state.config.cellSize * state.config.gameWidth,
          h: state.config.cellSize * state.config.gameHeight,
          color: state.colors.foodColor
        });
        await clearScreen({
          _falangGlobal,
          state
        });
        await DrawFood({
          _falangGlobal,
          state
        });
        await drawSnakeSquare({
          _falangGlobal,
          point: newSnakePoint,
          color: state.colors.cellColor,
          config: state.config
        });
      } else {
        state.snake.body.unshift(nextPoint);
        await drawSnakeSquare({
          _falangGlobal,
          point: nextPoint,
          color: state.colors.cellColor,
          config: state.config
        });
        if (state.snake.x == state.food.x && state.snake.y == state.food.y) {
          let newFoodPoint = await getNewFoodPoint({
            _falangGlobal,
            state
          });
          state.food = newFoodPoint;
          await DrawFood({
            _falangGlobal,
            state
          });
        } else {
          let oldPoint = state.snake.body.pop();
          if (!oldPoint)
            throw new Error("");
          await drawSnakeSquare({
            _falangGlobal,
            point: oldPoint,
            color: state.colors.backgroundColor,
            config: state.config
          });
          await _falangGlobal.apis.GameApi.Drawing.DrawCircle({
            x: oldPoint.x * state.config.cellSize + state.config.cellSize / 2,
            y: oldPoint.y * state.config.cellSize + state.config.cellSize / 2,
            r: state.config.pointRadius,
            color: state.colors.pointColor
          });
        }
      }
    } while (true);
  }

  // src/index.ts
  var run = () => {
    let canvas = document.getElementById("canvas");
    if (!canvas)
      throw new Error();
    let ctx = canvas.getContext("2d");
    if (!ctx)
      throw new Error("");
    ctx.strokeStyle = "transparent";
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    canvas.width = 800;
    canvas.height = 600;
    let pressedButton = 0 /* None */;
    document.addEventListener("keydown", (e) => {
      if (e.keyCode == 87 || e.keyCode == 38) {
        pressedButton = 3 /* Top */;
      } else if (e.keyCode == 65 || e.keyCode == 37) {
        pressedButton = 1 /* Left */;
      } else if (e.keyCode == 83 || e.keyCode == 40) {
        pressedButton = 4 /* Bottom */;
      } else if (e.keyCode == 68 || e.keyCode == 39) {
        pressedButton = 2 /* Right */;
      }
    });
    main({
      config: {
        cellSize: 40,
        foodSize: 15,
        pointRadius: 6,
        gameWidth: 20,
        gameHeight: 15
      },
      _falangGlobal: {
        apis: {
          GameApi: {
            Application: {
              GetButtonsState: async () => {
                let btn = pressedButton;
                pressedButton = 0 /* None */;
                console.log("btn");
                return {
                  bottom: btn == 4 /* Bottom */,
                  top: btn == 3 /* Top */,
                  left: btn == 1 /* Left */,
                  right: btn == 2 /* Right */
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
              DrawRect: async ({ x, y, h, w, color }) => {
                ctx.fillStyle = `rgb(${color.r},${color.g},${color.b})`;
                ctx.fillRect(x, y, w, h);
              },
              DrawCircle: async ({ color, r, x, y }) => {
                ctx.fillStyle = `rgb(${color.r},${color.g},${color.b})`;
                ctx.beginPath();
                ctx.arc(x, y, r, 0, 2 * Math.PI);
                ctx.fill();
              }
            }
          }
        }
      }
    });
  };
  run();
})();
