module Main where

import Lexer
import System.IO

main :: IO ()
main = do
	contents <- readFile' "sugar/toy.sug"
	iterFile contents
	putStrLn contents
