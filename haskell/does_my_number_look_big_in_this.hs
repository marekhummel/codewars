-- https://www.codewars.com/kata/5287e858c6b5a9678200083c

narcissistic :: Integral a => a -> Bool
narcissistic n =
  let ex = floor (logBase 10 (fromIntegral n :: Double)) + 1
  in sumOfPoweredDigits ex n == n


sumOfPoweredDigits :: Integral a => a -> a -> a
sumOfPoweredDigits _ 0 = 0
sumOfPoweredDigits ex n =
  let (d, m) = divMod n 10
  in m ^ ex + sumOfPoweredDigits ex d
