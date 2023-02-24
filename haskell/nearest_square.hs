-- https://www.codewars.com/kata/5a805d8cafa10f8b930005ba

import           Data.List
import           Testing   (assert)

nearestSquare :: Int -> Int
nearestSquare n =
  let sn = sqrt $ fromIntegral n in
    minimumBy (\a b -> compare (n - a) (b - n)) [floor sn ^ 2, ceiling sn ^ 2]


tests :: IO()
tests = do
  assert 0 (nearestSquare 0)
  assert 1 (nearestSquare 2)
  assert 9 (nearestSquare 8)


