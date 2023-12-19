use strict;
use warnings;
use v5.16;

my $string = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
my $count_A = $string =~ tr/A//; # when a replacement list is not provided, tr/// returns the number of occurences
my $count_C = $string =~ tr/C//;
my $count_G = $string =~ tr/G//;
my $count_T = $string =~ tr/T//;

print "$count_A $count_C $count_G $count_T\n";