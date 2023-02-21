-- https://www.codewars.com/kata/54da539698b8a2ad76000228

import           Testing (assert)

isValidWalk :: [Char] -> Bool
isValidWalk walk = length (take 11 walk) == 10 && (count 'n' walk == count 's' walk) && (count 'e' walk == count 'w' walk)
  where
    count :: Char -> [Char] -> Int
    count step = length . filter (==step)


tests :: IO()
tests = do
  assert True (isValidWalk ['n','s','n','s','n','s','n','s','n','s'])
  assert False (isValidWalk ['n','s','n','s','n','s','n','s','n','n'])
  assert False (isValidWalk ['n','s'])
  assert False (isValidWalk (repeat 'n'))
  assert True (isValidWalk ['n','s','e','w','n','s','e','w','n','s'])
