# k-nucleotide

Testing Rust with the [Alioth k-nucleotide benchmark](http://shootout.alioth.debian.org/u64q/performance.php?test=knucleotide)

```
$ time ./knucleotide < 250m.fasta 
A 30.000
T 30.000
G 20.000
C 20.000

AA 9.002
TA 9.001
TT 8.999
AT 8.998
GT 6.004
AG 6.000
AC 6.000
TC 6.000
GA 6.000
TG 6.000
CA 5.998
CT 5.998
CC 4.002
CG 4.002
GG 3.999
GC 3.997

1503927	GGT
454564	GGTA
45547	GGTATT
893	GGTATTTTAATT
893	GGTATTTTAATTTATAGT

real	2m15.612s
user	12m44.788s
sys	0m1.796s
```

On a ~240 MB FASTA file input, the multi-tasking version runs in ~140 seconds.
This is using about 1 thread per core...  Of course, on my 8-CPU system
the C++ version runs in about 2.3 seconds. :P


