/**
 * Generated by Falang
 * Document: DrawFood
 * Id: gFRB0Owe8fCqXE8tUqgfN
 **/
#[allow(non_snake_case)]
#[allow(unused_mut)]
// f:+icon:function:52uYxdCVKxGQj9iZAv5Iw
/**
 * 
 **/
pub fn DrawFood(state: &crate::falang::State::GameState, _apis: &mut dyn crate::falang::falang_global::Apis) -> ()
{
  // f:+icon:call_api:CzavJabM-Qart76uOglVG
  _apis.GameApi_Drawing_DrawCircle(state.food.x * state.config.cellSize + (1) as i32 / (2) as i32 + state.config.cellSize / (2) as i32, state.food.y * state.config.cellSize + state.config.cellSize / (2) as i32, state.config.foodSize, &state.colors.foodColor);
  // f:-icon:call_api:CzavJabM-Qart76uOglVG
  
}
// f:-icon:function:52uYxdCVKxGQj9iZAv5Iw

