// single tasking k-nucleotide

import io::reader_util;

use std;
import std::map;
import std::map::hashmap;
import std::sort;

// given a map, print a sorted version of it
fn sort_and_print(mm: hashmap<[u8], uint>, total: uint) { 
   fn pct(xx: uint, yy: uint) -> float {
      ret (xx as float) * 100f / (yy as float);
   }

   fn le_by_val<TT: copy, UU: copy>(kv0: (TT,UU), kv1: (TT,UU)) -> bool {
      let (_, v0) = kv0;
      let (_, v1) = kv1;
      ret v0 >= v1;
   }

   fn le_by_key<TT: copy, UU: copy>(kv0: (TT,UU), kv1: (TT,UU)) -> bool {
      let (k0, _) = kv0;
      let (k1, _) = kv1;
      ret k0 <= k1;
   }

   fn sortKV<TT: copy, UU: copy>(orig: [(TT,UU)]) -> [(TT,UU)] {
      ret sort::merge_sort(le_by_val, sort::merge_sort(le_by_key, orig));
   }

   let mut pairs = [];

   mm.each(fn&(key: [u8], val: uint) -> bool {
      pairs += [(key, pct(val, total))];
      ret true;
   });

   let pairs_sorted = sortKV(pairs);
   
   pairs_sorted.each(fn@(kv: ([u8], float)) -> bool unsafe {
      let (k,v) = kv;
      io::println(#fmt["%s %0.3f", str::to_upper(str::unsafe::from_bytes(k)), v]);
      ret true;
   });
}

// given a map, search for the frequency of a pattern
fn find(mm: hashmap<[u8], uint>, key: str) -> uint {
   alt mm.find(str::bytes(str::to_lower(key))) {
      option::none      { ret 0u; }
      option::some(num) { ret num; }
   }
}

// given a map, increment the counter for a key
fn update_freq(mm: hashmap<[u8], uint>, key: [u8]) {
   alt mm.find(key) {
      option::none      { mm.insert(key, 1u      ); }
      option::some(val) { mm.insert(key, 1u + val); }
   }
}

// given a [u8], for each window call a function
// i.e., for "hello" and windows of size four,
// run it("hell") and it("ello"), then return "llo"
fn windows_with_carry(bb: [const u8], nn: uint, it: fn(window: [u8])) -> [u8] {
   let mut ii = 0u;

   let len = vec::len(bb);
   while ii < len - (nn - 1u) {
      it(vec::slice(bb, ii, ii+nn));
      ii += 1u;
   }

   ret vec::slice(bb, len - (nn - 1u), len); 
}


// given a FASTA file on stdin, process sequence THREE
fn main () {
   fn map() -> hashmap<[u8], uint> {
      ret map::bytes_hash();
   }

   let sizes: [uint]                = [1u,2u,3u,4u,6u,12u,18u];
   let freqs: [hashmap<[u8], uint>] = [map(),map(),map(),map(),map(),map(),map()];
   let carry: [mut [u8]]            = [mut [],[],[],[],[],[],[],[]];
   let tot:   [mut uint]            = [mut 0u,0u,0u,0u,0u,0u,0u,0u];
   
   // latch stores true after we've started
   // reading the sequence of interest
   let mut proc_mode = false;

   let rdr = io::stdin();

   while !rdr.eof() {
      let line: str = rdr.read_line();

      if str::len(line) == 0u { cont; }

      alt (line[0], proc_mode) {

         // start processing if this is the one
         ('>' as u8, false) {
            alt str::find_str_from(line, "THREE", 1u) {
               option::some(_) { proc_mode = true; }
               option::none    { }
            }
         }

         // break our processing
         ('>' as u8, true) { break; }

         // process the sequence for k-mers
         (_, true) {
            let line_b = str::bytes(line);

            // FIXME: this, and subroutines, is where we really spend our time
/*
            for sizes.eachi { |ii, sz|
               let mut buffer = carry[ii] + line_b;
               carry[ii] = windows_with_carry(buffer, sz, { |window|
                  tot[ii] += 1u; update_freq(freqs[ii], window);
               });
            }
*/
            let mut ii = 1u;
            let mut sz = sizes[ii];
               let mut buffer = carry[ii] + line_b;
               carry[ii] = windows_with_carry(buffer, sz, { |window|
                  tot[ii] += 1u; update_freq(freqs[ii], window);
               });

            ii = 6u;
            sz = sizes[ii];
               let mut buffer = carry[ii] + line_b;
               carry[ii] = windows_with_carry(buffer, sz, { |window|
                  tot[ii] += 1u; update_freq(freqs[ii], window);
               });
         }

         // whatever
         _ { }
      }
   }

//   sort_and_print(freqs[0], tot[0]);
//   io::println("");

   sort_and_print(freqs[1], tot[1]);
   io::println("");

//   io::println(#fmt["%u\t%s", find(freqs[2], "GGT"), "GGT"]);
//   io::println(#fmt["%u\t%s", find(freqs[3], "GGTA"), "GGTA"]);
//   io::println(#fmt["%u\t%s", find(freqs[4], "GGTATT"), "GGTATT"]);
//   io::println(#fmt["%u\t%s", find(freqs[5], "GGTATTTTAATT"), "GGTATTTTAATT"]);
   io::println(#fmt["%u\t%s", find(freqs[6], "GGTATTTTAATTTATAGT"), "GGTATTTTAATTTATAGT"]);
}

