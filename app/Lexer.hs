module Lexer where

import System.IO
import Control.DeepSeq
import Flow

-- Token Types
data Token = 
	Char Char |
	Number Int |

	-- Math
	Plus |
	Minus |
	Asterisk |
	Slash |
	Equals |

	-- Misc
	Bang |
	SemiColon


-- Opens and reads a file
openAndRead :: String -> IO String 
openAndRead fileName = $!! <| hGetContents <| openFile fileName ReadMode |> hClose

-- Iterates contents
iterFile :: String
iterFile contents = map matchCharToToken contents

-- Map a char for `iterFile`
matchCharToToken :: Char -> Token
matchCharToToken c | isAlpha c = Char c
				   | c == '+'  = Plus
				   | c == '-'  = Minus
				   | c == '*'  = Asterisk
				   | c == '/'  = Slash
				   | c == '='  = Equals
				   | c == '!'  = Bang
				   | c == ';'  = SemiColon
