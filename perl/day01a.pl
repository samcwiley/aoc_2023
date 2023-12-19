use v5.16;
use warnings;
use strict;

my $sum = 0;
for (<>) {
    /([1-9])/;
    $sum += ($1 * 10);
    /.*([1-9])/;
    $sum += $1;
}

say $sum;