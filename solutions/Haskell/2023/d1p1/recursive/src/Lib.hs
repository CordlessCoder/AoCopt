{-# LANGUAGE BangPatterns #-}
module Lib (adventOfCode) where

import Data.Char

adventOfCode :: String -> Int
adventOfCode = readFirstDigit

readFirstDigit :: String -> Int
readFirstDigit ![]                 = 0
readFirstDigit !(c:cs) | isDigit c = case readSecondDigit (digitToInt c) cs of
                                      (n, r) -> (digitToInt c) * 10 + n + (readFirstDigit r)
                       | otherwise = readFirstDigit cs

readSecondDigit :: Int -> String -> (Int, String)
readSecondDigit n ('\n':cs)          = (n, cs)
readSecondDigit n (c:cs) | isDigit c = readSecondDigit (digitToInt c) cs
                         | otherwise = readSecondDigit n cs
