module Testing where

assert :: (Show a, Eq a) => a -> a -> IO ()
assert expected actual
  | expected == actual = print "Success"
  | otherwise          = print $ "Fail: Expected '" ++ show expected ++ "', but was '" ++ show actual ++ "'"
