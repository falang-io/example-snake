/**
 * Generated by Falang
 * Document: isGameOver
 * Id: Xf2XR9efm2JETsaCaeGA5
 **/
import { FalangGlobal } from './_falang';
import { GameState } from './State';
// f:+icon:function:yjmYQyyF_Ix6ogr8CT6Ua
/**
 * 
 **/
export interface IisGameOverParams {
  state: GameState;
  _falangGlobal: FalangGlobal;
}
export async function isGameOver(_params: IisGameOverParams): Promise<boolean> {
  const _falangGlobal = _params._falangGlobal;
  let state: GameState = JSON.parse(JSON.stringify(_params.state));
  let returnValue: boolean = false;
  // f:+icon:if:7vFBpQLqMAz29RmBlwmBF
  if (state.snake.dirX == 0 && state.snake.dirY == 0) {
    return returnValue;
  } else {
    // f:+icon:foreach:2KgEVo-jXSe7hZ3GaKxrg
    for (const item of state.snake.body) {
      // f:+icon:if:lHpC1piTjSnRL_S3FQTtp
      if (item.x == state.snake.x && item.y == state.snake.y) {
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
  return returnValue;
}
// f:-icon:function:yjmYQyyF_Ix6ogr8CT6Ua

