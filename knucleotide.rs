import io::reader_util;

use std;
import std::map;
import std::map::hashmap;
import std::sort;

fn main () {
   fn make_map() -> hashmap<str, uint> {
      ret map::hash_from_strs([]);
   }

   let freqs1 = make_map();
   let freqs2 = make_map();
/*
   let freqs3 = make_map();
   let freqs4 = make_map();
   let freqs6 = make_map();
   let freqs12 = make_map();
   let freqs18 = make_map();
*/

   let mut carry1 = "";
   let mut carry2 = "";
/*
   let mut carry3 = "";
   let mut carry4 = "";
   let mut carry6 = "";
   let mut carry12 = "";
   let mut carry18 = "";
*/

   let mut tot1 = 0u;
   let mut tot2 = 0u;
/*
   let mut tot3 = 0u;
   let mut tot4 = 0u;
   let mut tot6 = 0u;
   let mut tot12 = 0u;
   let mut tot18 = 0u;
*/

   // increment one counter
   let update_freq = fn@(mm: hashmap<str, uint>, key: str) {

      alt mm.find(key) {
         option::none      { mm.insert(key, 1u      ); }
         option::some(val) { mm.insert(key, 1u + val); }
      }
   };

   // iterate through a window of a string,
   // i.e., for "hello" and n=4, run it("hell"), and it("ello")
   //       and return "llo"
   fn windowsWithCarry(ss: str, nn: uint, it: fn(window: str)) -> str {
      let mut ii = 0u;

      let len = str::len(ss);
      while ii < len - (nn - 1u) {
         it(ss.slice(ii, ii+nn));
         ii += 1u;
         io::println("+");
      }

      let carry = ss.slice(len - (nn - 1u), len); 
      io::println("{" + carry + "}");
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
               carry1 = windowsWithCarry(carry1 + line, 1u, {|window| tot1 += 1u; update_freq(freqs1, window); });
               carry2 = windowsWithCarry(carry2 + line, 2u, {|window| tot2 += 1u; update_freq(freqs2, window); });
         }

         // whatever
         _ { }
      }
   }

   fn le_by_val<TT,UU>(kv0: (TT,UU), kv1: (TT,UU)) -> bool {
      let (_, v0) = kv0;
      let (_, v1) = kv1;
      ret v0 >= v1;
   }

   fn le_by_key<TT,UU>(kv0: (TT,UU), kv1: (TT,UU)) -> bool {
      let (k0, _) = kv0;
      let (k1, _) = kv1;
      ret k0 <= k1;
   }

   fn sortKV<TT,UU>(orig: [(TT,UU)]) -> [(TT,UU)] {
      ret sort::merge_sort(le_by_val, sort::merge_sort(le_by_key, orig));
   }

   let mut kv1 = [];
   let mut kv2 = [];

   fn fdiv(xx: uint, yy: uint) -> float {
      ret (xx as float) / (yy as float);
   }
   freqs1.each(fn&(key: str, val: uint) -> bool { kv1 += [(key, fdiv(val, tot1))]; ret true });
   freqs2.each(fn&(key: str, val: uint) -> bool { kv2 += [(key, fdiv(val, tot2))]; ret true });

   let kv1_sorted = sortKV(kv1);
   let kv2_sorted = sortKV(kv2);

   kv1_sorted.each(fn@(kv: (str, float)) -> bool { let (k,v) = kv; io::println(#fmt["%s %f", k, v]); ret true});
   kv2_sorted.each(fn@(kv: (str, float)) -> bool { let (k,v) = kv; io::println(#fmt["%s %f", k, v]); ret true});
      
}


