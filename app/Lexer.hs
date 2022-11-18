module Lexer where

import System.IO
import Control.DeepSeq
import Flow

-- Opens and reads a file
openAndRead :: String -> IO String 
openAndRead fileName = openFile fileName ReadMode |> hGetContents |> $!!
