// single tasking k-nucleotide

import io::reader_util;

use std;
import std::map;
import std::map::hashmap;
import std::sort;

fn main () unsafe {
   fn make_map() -> hashmap<[u8], uint> {
      ret map::bytes_hash();
   }

   // FIXME: combine these
   let freqs1 = make_map();
   let freqs2 = make_map();
   let freqs3 = make_map();
   let freqs4 = make_map();
   let freqs6 = make_map();
   let freqs12 = make_map();
   let freqs18 = make_map();

   let mut carry1 = [];
   let mut carry2 = [];
   let mut carry3 = [];
   let mut carry4 = [];
   let mut carry6 = [];
   let mut carry12 = [];
   let mut carry18 = [];

   let mut tot1 = 0u;
   let mut tot2 = 0u;
   let mut tot3 = 0u;
   let mut tot4 = 0u;
   let mut tot6 = 0u;
   let mut tot12 = 0u;
   let mut tot18 = 0u;

   // increment one counter
   let update_freq = fn@(mm: hashmap<[u8], uint>, key: [u8]) {
    //let KEY = key.to_upper();
      let KEY = key;

      alt mm.find(KEY) {
         option::none      { mm.insert(KEY, 1u      ); }
         option::some(val) { mm.insert(KEY, 1u + val); }
      }
   };

   // iterate through a window of a string,
   // i.e., for "hello" and n=4, run it("hell"), and it("ello")
   //       and return "llo"
   fn windowsWithCarry(bb: [u8], nn: uint, it: fn(window: [u8])) -> [u8] {
      //let bb = str::bytes(ss);
      let mut ii = 0u;

      let len = vec::len(bb);
      while ii < len - (nn - 1u) {
         it(vec::view(&bb, ii, ii+nn));
         ii += 1u;
      }

      let carry = vec::view(&bb, len - (nn - 1u), len); 
      ret carry;
   }

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
               option::none { }
            }
         }

         // break our processing
         ('>' as u8, true) {
            break;
         }
         // process the sequence for k-mers
         (_, true) {
               let line_b = str::bytes(line);

               // FIXME: combine these
               carry1 = windowsWithCarry(carry1 + line_b, 1u, {|window| tot1 += 1u; update_freq(freqs1, window); });
               carry2 = windowsWithCarry(carry2 + line_b, 2u, {|window| tot2 += 1u; update_freq(freqs2, window); });
               carry3 = windowsWithCarry(carry3 + line_b, 3u, {|window| tot3 += 1u; update_freq(freqs3, window); });
               carry4 = windowsWithCarry(carry4 + line_b, 4u, {|window| tot4 += 1u; update_freq(freqs4, window); });
               carry6 = windowsWithCarry(carry6 + line_b, 6u, {|window| tot6 += 1u; update_freq(freqs6, window); });
               carry12 = windowsWithCarry(carry12 + line_b, 12u, {|window| tot12 += 1u; update_freq(freqs12, window); });
               carry18 = windowsWithCarry(carry18 + line_b, 18u, {|window| tot18 += 1u; update_freq(freqs18, window); });
         }

         // whatever
         _ { }
      }
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

   let mut kv1 = [];
   let mut kv2 = [];

   fn pct(xx: uint, yy: uint) -> float {
      ret (xx as float) * 100f / (yy as float);
   }
   freqs1.each(fn&(key: [u8], val: uint) -> bool { kv1 += [(key, pct(val, tot1))]; ret true });
   freqs2.each(fn&(key: [u8], val: uint) -> bool { kv2 += [(key, pct(val, tot2))]; ret true });

   let kv1_sorted = sortKV(kv1);
   let kv2_sorted = sortKV(kv2);

   kv1_sorted.each(fn@(kv: ([u8], float)) -> bool { let (k,v) = kv; io::println(#fmt["%s %0.3f", str::to_upper(str::unsafe::from_bytes(k)), v]); ret true});
   io::println("");
   kv2_sorted.each(fn@(kv: ([u8], float)) -> bool { let (k,v) = kv; io::println(#fmt["%s %0.3f", str::to_upper(str::unsafe::from_bytes(k)), v]); ret true});
   io::println("");

   fn find(mm: hashmap<[u8], uint>, key: str) -> uint {
      alt mm.find(str::bytes(str::to_lower(key))) {
         option::none      { ret 0u; }
         option::some(num) { ret num; }
      }
   }

   io::println(#fmt["%u\t%s", find(freqs3, "GGT"), "GGT"]);
   io::println(#fmt["%u\t%s", find(freqs4, "GGTA"), "GGTA"]);
   io::println(#fmt["%u\t%s", find(freqs6, "GGTATT"), "GGTATT"]);
   io::println(#fmt["%u\t%s", find(freqs12, "GGTATTTTAATT"), "GGTATTTTAATT"]);
   io::println(#fmt["%u\t%s", find(freqs18, "GGTATTTTAATTTATAGT"), "GGTATTTTAATTTATAGT"]);
      
}

