/**
 * Generated by Falang
 * Document: drawSnakeSquare
 * Id: JWQudFlU2WMpaPUF9FajM
 **/
#[allow(non_snake_case)]
#[allow(unused_mut)]
// f:+icon:function:K26d9OrusDeFwiQif8nMs
/**
 * 
 **/
pub fn drawSnakeSquare(point: &crate::falang::State::Point, color: &crate::falang::State::Color, config: &crate::falang::State::Config, _apis: &mut dyn crate::falang::falang_global::Apis) -> ()
{
  // f:+icon:call_api:inPlwfqnVyKbuKIARZ0aK
  _apis.GameApi_Drawing_DrawRect(point.x * config.cellSize, point.y * config.cellSize, config.cellSize - (2) as i32, config.cellSize - (2) as i32, &color);
  // f:-icon:call_api:inPlwfqnVyKbuKIARZ0aK
  
}
// f:-icon:function:K26d9OrusDeFwiQif8nMs

