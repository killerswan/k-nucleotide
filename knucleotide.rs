// single tasking k-nucleotide

import io::reader_util;

use std;
import std::map;
import std::map::hashmap;
import std::sort;
import std::arc;
import std::arc::arc;

// given a map, print a sorted version of it
fn sort_and_fmt(mm: hashmap<[u8], uint>, total: uint) -> str { 
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

   // sort by key, then by value
   fn sortKV<TT: copy, UU: copy>(orig: [(TT,UU)]) -> [(TT,UU)] {
      ret sort::merge_sort(le_by_val, sort::merge_sort(le_by_key, orig));
   }

   let mut pairs = [];

   // map -> [(k,%)]
   mm.each(fn&(key: [u8], val: uint) -> bool {
      pairs += [(key, pct(val, total))];
      ret true;
   });

   let pairs_sorted = sortKV(pairs);
   
   let mut buffer = "";

   pairs_sorted.each(fn&(kv: ([u8], float)) -> bool unsafe {
      let (k,v) = kv;
      buffer += (#fmt["%s %0.3f\n", str::to_upper(str::unsafe::from_bytes(k)), v]);
      ret true;
   });

   ret buffer;
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

fn make_sequence_processor(sz: uint, from_parent: comm::port<arc<[[u8]]>>, to_parent: comm::chan<str>) {
   
   let freqs: hashmap<[u8], uint> = map::bytes_hash();
   let mut carry: [u8] = [];
   let mut total: uint = 0u;

   let lines_arc = comm::recv(from_parent);
   let lines: [[u8]] = *arc::get::<[[u8]]>(&lines_arc);

   for lines.each { |line|
      if vec::len(line) != 0u {
         carry = windows_with_carry(carry + line, sz, { |window|
            update_freq(freqs, window);
            total += 1u;
         });
      }
   }

   let buffer = alt sz { 
       1u { sort_and_fmt(freqs, total) }
       2u { sort_and_fmt(freqs, total) }
       3u { #fmt["%u\t%s", find(freqs, "GGT"), "GGT"] }
       4u { #fmt["%u\t%s", find(freqs, "GGTA"), "GGTA"] }
       6u { #fmt["%u\t%s", find(freqs, "GGTATT"), "GGTATT"] }
      12u { #fmt["%u\t%s", find(freqs, "GGTATTTTAATT"), "GGTATTTTAATT"] }
      18u { #fmt["%u\t%s", find(freqs, "GGTATTTTAATTTATAGT"), "GGTATTTTAATTTATAGT"] }
        _ { "" }
   };

   //comm::send(to_parent, #fmt["yay{%u}", sz]);
   comm::send(to_parent, buffer);
}

// given a FASTA file on stdin, process sequence THREE
fn main () {

   // initialize each sequence sorter
   let sizes = [1u,2u,3u,4u,6u,12u,18u];
   let from_child = vec::map (sizes, { |_sz|     comm::port() });
   let to_parent  = vec::mapi(sizes, { |ii, _sz| comm::chan(from_child[ii]) });
   let to_child   = vec::mapi(sizes, fn@(ii: uint, sz: uint) -> comm::chan<arc<[[u8]]>> {
      ret task::spawn_listener { |from_parent|
         make_sequence_processor(sz, from_parent, to_parent[ii]);
      };
   });
         
   
   // latch stores true after we've started
   // reading the sequence of interest
   let mut proc_mode = false;

   let rdr = io::stdin();
   let file = rdr.read_whole_stream();
   let lines = vec::split(file, { |c| c == '\n' as u8 });

   let mut data_start = 0u;
   let mut data_end   = 0u;

   for lines.eachi { |jj, line|
      if vec::len(line) == 0u { cont; }

      alt (line[0], proc_mode) {

         // start processing if this is the one
         ('>' as u8, false) {
            if vec::len(line) >= 6u  &&
               line[1] == 'T' as u8 &&
               line[2] == 'H' as u8 &&
               line[3] == 'R' as u8 &&
               line[4] == 'E' as u8 &&
               line[5] == 'E' as u8
            {
               proc_mode = true;
               data_start = jj + 1u;
            }
         }

         // break our processing
         ('>' as u8, true) {
            data_end = jj;
            break;
         }

         // whatever
         _ { }
      }
   }

   // in case we didn't find an 'end', but went through all the lines
   if proc_mode == true || data_end == 0u { data_end = vec::len(lines); }

   // grab the relevant FASTA sequence as an ARC reference
   let sequence = vec::slice(lines, data_start, data_end);
   let sequence_arc = arc::arc(sequence);

   // send the the reference (or rather a clone of it) to each child
   for sizes.eachi { |ii, _sz|
      comm::send(to_child[ii], arc::clone(&sequence_arc));
   }

   // now fetch and print result messages
   for sizes.eachi { |ii, _sz|
      io::println(comm::recv(from_child[ii]));
   }
}

