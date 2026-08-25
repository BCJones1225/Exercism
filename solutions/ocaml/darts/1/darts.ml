let score (x: float) (y: float): int =
  let dis = Float.sqrt (x ** 2. +. y ** 2.) in
  match () with
  | _ when dis > 10.               -> 0
  | _ when dis > 5. && dis <= 10.  -> 1
  | _ when dis > 1. && dis <= 5.   -> 5
  | _ when dis <= 1.               -> 10