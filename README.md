# k-nucleotide

Testing Rust with the [Alioth k-nucleotide benchmark](http://shootout.alioth.debian.org/u64q/performance.php?test=knucleotide)

```
$ ./knucleotide < knucleotide-input.txt 
A 30.279
T 30.113
G 19.835
C 19.773

AA 9.161
AT 9.138
TA 9.108
TT 9.060
CA 6.014
GA 5.996
AG 5.993
AC 5.988
TG 5.987
GT 5.967
TC 5.958
CT 5.948
GG 3.944
GC 3.928
CG 3.910
CC 3.899

1474	GGT
459	GGTA
49	GGTATT
1	GGTATTTTAATT
1	GGTATTTTAATTTATAGT
```

```
$ time diff -u knucleotide-output.txt <(./knucleotide < knucleotide-input.txt)

real	0m0.682s
user	0m0.644s
sys	0m0.032s
```

Note that right now the algorithm is single threaded, and
for the larger sets of FASTA data weighs in at around **6m 55s**.
Lots of room for improvement, here...

