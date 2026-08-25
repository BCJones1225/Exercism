let validate n =
  if n < 0 then false
  else
    let s = string_of_int n in
    let num_digits = String.length s in
    let rec pow base exp acc =
      if exp = 0 then acc
      else pow base (exp - 1) (acc * base)
    in
    let sum =
      let acc = ref 0 in
      for i = 0 to num_digits - 1 do
        let digit = int_of_string (String.sub s i 1) in
        acc := !acc + pow digit num_digits 1
      done;
      !acc
    in
    sum = n   