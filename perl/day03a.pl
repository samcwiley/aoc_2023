use v5.16;
use strict;
use warnings;

my @grid = map { chomp; ".$_." } <>;
push (@grid, '.' x length $grid[0]);
unshift (@grid, '.' x length $grid[0]);

my $part1 = 0;
#my %gears;

foreach my $line (1 .. $#grid - 1) { # $#grid returns index of the last element in the array
    while ($grid[$line] =~ m/(\d+)/g) {
        my $num = $1;
        my $x = pos($grid[$line]) - length($num) - 1;   # left edge of search block

        # Search surrounding block for parts and gears
        my $found = 0;
        foreach my $y ($line - 1 .. $line + 1) {
            my $str = substr($grid[$y], $x, length($num) + 2);

            $found = 1 if ($str =~ m/[^.0-9]/);
            #push($gears{$y, $x + pos($str)}->@*, $num)  while ($str =~ m#\*#g);
        }

        $part1 += $num if ($found);
    }
}
say $part1;