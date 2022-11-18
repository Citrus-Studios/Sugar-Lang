module Main where

import Lexer
import System.IO

main :: IO ()
main = do
	contents <- openAndRead "sugar/toy.sug"
	putStrLn contents
