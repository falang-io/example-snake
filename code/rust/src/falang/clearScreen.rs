/**
 * Generated by Falang
 * Document: clearScreen
 * Id: MfN5AOPHfZqUJ2ngOu3-Z
 **/
#[allow(non_snake_case)]
#[allow(unused_mut)]
// f:+icon:function:XhClfyJF7x2w7pFQxy4O_
/**
 * 
 **/
pub fn clearScreen(state: &crate::falang::State::GameState, _apis: &mut dyn crate::falang::falang_global::Apis) -> ()
{
  // f:+icon:call_api:bXiI5Jw1fSTQY0GMDdYVH
  _apis.GameApi_Drawing_DrawRect((0) as i32, (0) as i32, state.config.cellSize * state.config.gameWidth, state.config.cellSize * state.config.gameHeight, &state.colors.backgroundColor);
  // f:-icon:call_api:bXiI5Jw1fSTQY0GMDdYVH
  // f:+icon:call_function:o8jolrJTPtrTu0_tgxsYc
  crate::falang::DrawPoints::DrawPoints(&state, _apis);
  // f:-icon:call_function:o8jolrJTPtrTu0_tgxsYc
  
}
// f:-icon:function:XhClfyJF7x2w7pFQxy4O_

