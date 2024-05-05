/**
 * Generated by Falang
 * Document: State
 * Id: kHyjfyZ38T2JD3yH7pzN3
 **/
// f:+icon:tree:m7IMRm5qZTdS75rvJ0-02
export interface Point {
  x: number
  y: number
}
export interface Snake {
  x: number
  y: number
  dirX: number
  dirY: number
  body: Point[]
}
export interface Color {
  r: number
  g: number
  b: number
}
export interface Config {
  cellSize: number
  foodSize: number
  pointRadius: number
  gameWidth: number
  gameHeight: number
}
export interface Colors {
  foodColor: Color
  cellColor: Color
  pointColor: Color
  backgroundColor: Color
}
export interface ButtonsState {
  left: boolean
  right: boolean
  top: boolean
  bottom: boolean
}
export interface GameState {
  colors: Colors
  snake: Snake
  food: Point
  config: Config
}
// f:-icon:tree:m7IMRm5qZTdS75rvJ0-02
