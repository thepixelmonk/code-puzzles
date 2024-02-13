module DescendingOrder where

import Data.List (sort)

descendingOrder :: Integer -> Integer
descendingOrder = fromDigits . reverse . sort . digits

digits :: Integral x => x -> [x]
digits 0 = []
digits x = digits (x `div` 10) ++ [x `mod` 10]

fromDigits = foldl addDigit 0
   where addDigit num d = 10*num + d