{-# LANGUAGE BangPatterns #-}

module Main (main) where

import Lib
import System.IO
import System.CPUTime

main :: IO ()
main = do
  s <- getContents'
  start <- getCPUTime
  let !res = adventOfCode s
  end <- getCPUTime
  putStrLn $ show res
  putStrLn . show $ (end - start) `div` 1000
