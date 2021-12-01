#!/usr/bin/fish
set FILE (cat $argv[1])
# part one
set sol 0
for i in (seq 2 (count $FILE))
	if test $FILE[$i] -gt $FILE[(math "$i - 1")]
		set sol (math "$sol + 1")
	end
end
echo "1: $sol"
# part two
set sol 0
for i in (seq 4 (count $FILE))
	if test $FILE[$i] -gt $FILE[(math "$i - 3")]
		set sol (math "$sol + 1")
	end
end
echo "2: $sol"
