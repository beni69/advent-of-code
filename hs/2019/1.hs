import Data.List
import Data.Char

trim = dropWhileEnd isSpace . dropWhile isSpace
split delim s = words [ if c == delim then ' ' else c | c <- s ]

sumList [] = 0
sumList (x:xs) = x + sumList xs

-- calcMass m = (floor (m / 3)) - 2
calcMass m = (-) (floor . (/) m $ 3) 2

main = do
    contents <- readFile "input.txt"
    print . sum . map calcMass . map (\x -> read x) . split '\n' . trim $ contents
