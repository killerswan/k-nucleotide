rustc -g shootout-k-nucleotide.rs
valgrind --tool=callgrind ./shootout-k-nucleotide < knucleotide-input.txt
kcachegrind callgrind*
