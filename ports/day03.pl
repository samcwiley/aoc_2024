use strict;
use warnings;

sub part1 {
    my ($input) = @_;
    my $sum = 0;
    
    while ($input =~ /mul\((\d+),(\d+)\)/g) {
        $sum += $1 * $2;
    }
    
    return $sum;
}

sub part2 {
    my ($input) = @_;
    my $part2 = 0;
    my $do = 1;
    
    foreach my $line (split /\n/, $input) {
        while ($line =~ m#(do(n't)?\(\)|mul\(\d{1,3},\d{1,3}\))#g) {
            if ($1 eq "do()") {
                $do = 1;
            } elsif ($1 eq "don't()") {
                $do = 0;
            } elsif ($do) {
                $1 =~ m#(\d+),(\d+)#;
                $part2 += $1 * $2;
            }
        }
    }
    return $part2;
}


my $filename = '../src/.inputs/input03.txt';
open my $fh, '<', $filename or die "Could not open file '$filename': $!";
my $input_string = do { local $/; <$fh> };
close $fh;

my $result = part1($input_string);
print "Result: $result\n";

my $result2 = part2($input_string);
print "Result Part 2: $result2\n";