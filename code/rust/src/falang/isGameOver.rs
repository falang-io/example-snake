/**
 * Generated by Falang
 * Document: isGameOver
 * Id: Xf2XR9efm2JETsaCaeGA5
 **/
#[allow(non_snake_case)]
#[allow(unused_mut)]
// f:+icon:function:yjmYQyyF_Ix6ogr8CT6Ua
/**
 * 
 **/
pub fn isGameOver(state: &crate::falang::State::GameState, _apis: &mut dyn crate::falang::falang_global::Apis) -> bool
{
  let mut returnValue: bool = false;
  // f:+icon:if:7vFBpQLqMAz29RmBlwmBF
  if state.snake.dirX == 0 && state.snake.dirY == 0 {
    return returnValue;
  } else {
    // f:+icon:foreach:2KgEVo-jXSe7hZ3GaKxrg
    for item in state.snake.body.iter() {
      // f:+icon:if:lHpC1piTjSnRL_S3FQTtp
      if item.x == state.snake.x && item.y == state.snake.y {
        // f:+icon:assign_var:TsI1YQfQ927aVVWWrYypG
        returnValue = true;
        // f:-icon:assign_var:TsI1YQfQ927aVVWWrYypG
        return returnValue;
      } else {
      }
      // f:-icon:if:lHpC1piTjSnRL_S3FQTtp
    }
    // f:-icon:foreach:2KgEVo-jXSe7hZ3GaKxrg
  }
  // f:-icon:if:7vFBpQLqMAz29RmBlwmBF
  
  returnValue
}
// f:-icon:function:yjmYQyyF_Ix6ogr8CT6Ua
