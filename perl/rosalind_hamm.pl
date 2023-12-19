use strict;
use warnings;
use v5.16;

my $string1 = "GAGCCTACTAACGGGAT";
my $string2 = "CATCGTAATGACGGCCT";

my $length = length $string1;
my $differences;

for my $i (0 .. $length - 1) {
    $differences++ if substr($string1, $i, 1) ne substr($string2, $i, 1);
}

print "Hamming distance: $differences\n";