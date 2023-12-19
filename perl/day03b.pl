use v5.16;
use strict;
use warnings;
use List::Util qw(sum product);

my @grid = map { chomp; ".$_." } <>;
push (@grid, '.' x length $grid[0]);
unshift (@grid, '.' x length $grid[0]);

my %gears;

foreach my $line (1 .. $#grid - 1) { # $#grid returns index of the last element in the array
    while ($grid[$line] =~ m/(\d+)/g) { # find all the digits
        my $num = $1;
        my $x = pos($grid[$line]) - length($num) - 1;   # left edge of digit

        foreach my $y ($line - 1 .. $line + 1) { #look at 3 lines surrounding the digit
            my $str = substr($grid[$y], $x, length($num) + 2); # take substrings that start at leftmost number to past the end of the number

            while ($str =~ m/\*/g) {
                my $current_position = $x + pos($str);
                my $coordinates_key = "$y, $current_position";
                
                my $gears_array_ref = $gears{$coordinates_key};
                if (defined $gears_array_ref) {
                    push(@$gears_array_ref, $num);
                } else {
                    # If the array does not exist at these coordinates, create it
                    $gears{$coordinates_key} = [$num];
                }
            }

        }
    }
}

my $answer = 0;

for my $coords (values %gears) {
    next unless @$coords == 2;  # Skip arrays with other than 2 elements
    my $product = $coords->[0] * $coords->[1];
    $answer += $product;
}

say $answer;