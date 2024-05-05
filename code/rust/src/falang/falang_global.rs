/**
 * Generated by Falang
 * Global functions
 **/
#[allow(non_snake_case)]
pub trait Apis {
  
  fn GameApi_Drawing_DrawCircle(&mut self,
    x: i32,
    y: i32,
    r: i32,
    color: &crate::falang::State::Color,
  ) -> ();
  
  fn GameApi_Drawing_DrawRect(&mut self,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    color: &crate::falang::State::Color,
  ) -> ();
  
  fn GameApi_Application_GetButtonsState(&mut self) -> crate::falang::State::ButtonsState;
  
  fn GameApi_Application_GetRandom(&mut self,
    min: i32,
    max: i32,
  ) -> i32;
  
  fn GameApi_Application_Sleep(&mut self) -> ();
  
}



