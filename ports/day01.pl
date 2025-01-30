my $filename = '../src/.inputs/input01.txt';
open my $fh, '<', $filename or die "Could not open file '$filename': $!"; # getting fh (file handle); $! is perl shorthand to return error message from last system or I/O operation
# my $input_string = do { local $/; <$fh> };

my (@list1, @list2, %counts); # list, list, hash for values


while (<$fh>) { # reads file by line
    m#(\d+)\s+(\d+)#; # (\d+) matches first number in $1, \s+ matches spaces, (\d+) matches second number; m#..# is one way to denote regex match
    push( @list1, $1 ); # pushing value to list one
    push( @list2, $2 ); # pushing value to list 2
    $counts{$2}++; # calling the hash map for counts and incrementing the 2nd value count by 1
}

close $fh; # gotta close your files

@list1 = sort { $a <=> $b } @list1; # normal sort in perl is lexicographic, so we need to provide our own sort for numbers
@list2 = sort { $a <=> $b } @list2; # $a and $b are variables provided by sort; <=> is a "spaceship operator", returns -1, 0, or 1 depending on greater than, less than, equal

sub part1 {
    my ($list1_ref, $list2_ref) = @_;  # Accept references to @list1 and @list2 as parameters to part1
    my @list1 = @$list1_ref;  # Dereference to get the actual arrays
    my @list2 = @$list2_ref;

    my $total = 0;
    for my $i (0 .. $#list1) {  # Loop over indices 
        $total += abs($list1[$i] - $list2[$i]);
    }

    return $total;
}

sub part2 {
    my ($list1_ref, $counts_ref) = @_;
    my @list1 = @$list1_ref;
    my %counts = %$counts_ref;  # Dereference the hash reference

    my $total = 0;
    for my $num (@list1) { # loop over actual numbers
        $total += $num * ($counts{$num} // 0);  # Multiply by occurrences, or default value 0 if $num is not in the hash
    }

    return $total;
}

print "Part 1: ", part1(\@list1, \@list2), "\n";
print "Part 2: ", part2(\@list1, \%counts), "\n";