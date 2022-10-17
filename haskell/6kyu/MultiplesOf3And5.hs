module MultiplesOf3And5 where

import Data.List (union)

solution :: Integer -> Integer
solution n = sum $ union [3,6 .. n-1] [5,10 .. n-1]