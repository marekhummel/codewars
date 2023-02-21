-- https://www.codewars.com/kata/5667e8f4e3f572a8f2000039

import           Data.Char (toLower, toUpper)
accum :: [Char] -> [Char]
accum str = help str 0
  where
    help :: [Char] -> Int -> [Char]
    help [] _    = []
    help [c] n   = repl c n
    help (c:s) n = repl c n ++ "-" ++ help s (n+1)

    repl :: Char -> Int -> [Char]
    repl c n = toUpper c : replicate n (toLower c)


-- accum s = intercalate "-" [ (toUpper c) : replicate i (toLower c) | c<-s | i<-[0..] ]
-- accum =  intercalate "-" . zipWith (\i c -> toUpper c : replicate (i-1) (toLower c)) [1..]
