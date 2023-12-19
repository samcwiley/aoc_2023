use v5.16;
use warnings;
use strict;

my %words = ("one"=>1, "two"=>2, "three"=>3, "four"=>4, "five"=>5, "six"=>6, "seven"=>7, "eight"=>8, "nine"=>9);
my $pattern = '[1-9]|' . join('|', keys %words);

my $sum = 0;
for (<>) {
    /($pattern)/;
    $sum += (($words{$1} // $1) * 10);
    /.*(pattern)/;
    $sum += ($words{$1} // $1);
}
say $sum;