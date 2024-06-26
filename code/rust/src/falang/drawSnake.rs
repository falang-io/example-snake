/**
 * Generated by Falang
 * Document: drawSnake
 * Id: 4MhR_gU7RXnpRlpwAE3YI
 **/
#[allow(non_snake_case)]
#[allow(unused_mut)]
// f:+icon:function:X3m1hzB1mjq9XeT9bIbhW
/**
 * 
 **/
pub fn drawSnake(state: &crate::falang::State::GameState, _apis: &mut dyn crate::falang::falang_global::Apis) -> ()
{
  // f:+icon:foreach:6eLWeAqS7GtULAKSTPWyc
  for point in state.snake.body.iter() {
    // f:+icon:call_api:RwaZmiYYLGBalHosUAIbm
    _apis.GameApi_Drawing_DrawSquare(point.x * state.config.cellSize, point.y * state.config.cellSize, state.config.cellSize - (1) as i32, state.config.cellSize - (1) as i32, &state.colors.cellColor);
    // f:-icon:call_api:RwaZmiYYLGBalHosUAIbm
  }
  // f:-icon:foreach:6eLWeAqS7GtULAKSTPWyc
  
}
// f:-icon:function:X3m1hzB1mjq9XeT9bIbhW

