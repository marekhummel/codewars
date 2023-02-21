-- https://www.codewars.com/kata/5656b6906de340bd1b0000ac

import           Data.List (nub, sort, union)
import           Testing   (assert)

longest :: [Char] -> [Char] -> [Char]
longest s1 s2 = sort $ nub (s1 ++ s2)
-- longest s1 s2 = sort . nub $ s1 ++ s2
-- longest s1 s2 = filter (`elem` s1++s2) ['a'..'z']

tests :: IO ()
tests = do
  assert "aehrsty" (longest "aretheyhere" "yestheyarehere" )
  assert "abcdefghilnoprstu" (longest "loopingisfunbutdangerous" "lessdangerousthancoding")
  assert "acefghilmnoprstuy" (longest "inmanylanguages" "theresapairoffunctions")
