use v5.16;
use warnings;
use strict;
use experimental 'switch';

my $sum = 0;
for (<>) {
    my ($id) = /Game (\d+)/;
    my ($max_red, $max_green, $max_blue) = (0, 0, 0);
    while (/(\d+) (red|green|blue)/g) {
        given ($2) { #match the color
            when ("red") {
                $max_red = $1 if $1 > $max_red;
            }
            when ("green") {
                $max_green = $1 if $1 > $max_green;
            }
            when ("blue") {
                $max_blue = $1 if $1 > $max_blue;
            }
        }
    }
    if ($max_red <= 12 && $max_green <= 13 && $max_blue <= 14) {
        $sum += $id;
    }
}
say $sum;