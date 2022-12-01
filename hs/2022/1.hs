import Data.Char
import Data.List (sort, sortOn)
import qualified Data.Text as T

main :: IO ()
main = do
  contents <- readFile "../js-ts/.cache/input-2022-1.txt"
  print "Part 1:"
  print . sum . take 1 . e $ contents
  print "Part 2:"
  print . sum . take 3 . e $ contents
  where
    e = reverse . sort . (map (sum . map ((read :: String -> Int) . T.unpack) . T.splitOn (T.pack "\n")) . T.splitOn (T.pack "\n\n") . T.dropWhileEnd isSpace . T.pack)
