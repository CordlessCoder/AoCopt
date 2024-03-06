{-# LANGUAGE BangPatterns #-}
module Lib (adventOfCode) where

import Data.Char

adventOfCode :: String -> Int
adventOfCode !s = sum $ map readCalibration (lines s)

readCalibration :: String -> Int
readCalibration s = readCalibration' $ map digitToInt (filter isDigit s)
  where
    readCalibration' [] = 0
    readCalibration' l  = (head l) * 10 + (last l)
