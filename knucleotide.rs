// single tasking k-nucleotide

import io::reader_util;

use std;
import std::map;
import std::map::hashmap;
import std::sort;

fn main () {
   fn make_map() -> hashmap<str, uint> {
      ret map::hash_from_strs([]);
   }

   // FIXME: combine these
   let freqs1 = make_map();
   let freqs2 = make_map();
   let freqs3 = make_map();
   let freqs4 = make_map();
   let freqs6 = make_map();
   let freqs12 = make_map();
   let freqs18 = make_map();

   let mut carry1 = "";
   let mut carry2 = "";
   let mut carry3 = "";
   let mut carry4 = "";
   let mut carry6 = "";
   let mut carry12 = "";
   let mut carry18 = "";

   let mut tot1 = 0u;
   let mut tot2 = 0u;
   let mut tot3 = 0u;
   let mut tot4 = 0u;
   let mut tot6 = 0u;
   let mut tot12 = 0u;
   let mut tot18 = 0u;

   // increment one counter
   let update_freq = fn@(mm: hashmap<str, uint>, key: str) {
      let KEY = key.to_upper();

      alt mm.find(KEY) {
         option::none      { mm.insert(KEY, 1u      ); }
         option::some(val) { mm.insert(KEY, 1u + val); }
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
      }

      let carry = ss.slice(len - (nn - 1u), len); 
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
               // FIXME: combine these
               carry1 = windowsWithCarry(carry1 + line, 1u, {|window| tot1 += 1u; update_freq(freqs1, window); });
               carry2 = windowsWithCarry(carry2 + line, 2u, {|window| tot2 += 1u; update_freq(freqs2, window); });
               carry3 = windowsWithCarry(carry3 + line, 3u, {|window| tot3 += 1u; update_freq(freqs3, window); });
               carry4 = windowsWithCarry(carry4 + line, 4u, {|window| tot4 += 1u; update_freq(freqs4, window); });
               carry6 = windowsWithCarry(carry6 + line, 6u, {|window| tot6 += 1u; update_freq(freqs6, window); });
               carry12 = windowsWithCarry(carry12 + line, 12u, {|window| tot12 += 1u; update_freq(freqs12, window); });
               carry18 = windowsWithCarry(carry18 + line, 18u, {|window| tot18 += 1u; update_freq(freqs18, window); });
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
   freqs1.each(fn&(key: str, val: uint) -> bool { kv1 += [(key, pct(val, tot1))]; ret true });
   freqs2.each(fn&(key: str, val: uint) -> bool { kv2 += [(key, pct(val, tot2))]; ret true });

   let kv1_sorted = sortKV(kv1);
   let kv2_sorted = sortKV(kv2);

   kv1_sorted.each(fn@(kv: (str, float)) -> bool { let (k,v) = kv; io::println(#fmt["%s %s", k, my_to_str_exact(v, 3u)]); ret true});
   io::println("");
   //kv2_sorted.each(fn@(kv: (str, float)) -> bool { let (k,v) = kv; io::println(#fmt["%s %0.3f", k, v]); ret true});
   kv2_sorted.each(fn@(kv: (str, float)) -> bool { let (k,v) = kv; io::println(#fmt["%s %s", k, my_to_str_exact(v, 3u)]); ret true});
   io::println("");
   io::println(#fmt["%u\t%s", freqs3.get("GGT"), "GGT"]);
   io::println(#fmt["%u\t%s", freqs4.get("GGTA"), "GGTA"]);
   io::println(#fmt["%u\t%s", freqs6.get("GGTATT"), "GGTATT"]);
   io::println(#fmt["%u\t%s", freqs12.get("GGTATTTTAATT"), "GGTATTTTAATT"]);
   io::println(#fmt["%u\t%s", freqs18.get("GGTATTTTAATTTATAGT"), "GGTATTTTAATTTATAGT"]);
      
}



// originally from float.rs
// 
fn my_to_str_common(num: float, num_total: uint, num_decimals: uint, exact: bool) -> str {
   import float::*;

    if is_NaN(num) { ret "NaN"; }
    if num == infinity { ret "inf"; }
    if num == neg_infinity { ret "-inf"; }

    let mut (num, sign) = if num < 0.0 { (-num, "-") } else { (num, "") };

    // truncated integer
    let trunc = num as uint;

    // decimal remainder
    let mut frac = num - (trunc as float);

    // stack of digits
    let mut fractionalParts = [];

    // note: this used to return right away without rounding, as "[-]num":
    //    if (frac < epsilon && !exact) || digits == 0u { ret accum; }
    // 
    // but given epsilon like in f64.rs, I don't see how it did much
    // const epsilon: f64 = 2.2204460492503131e-16_f64;

    let mut ii = num_decimals;
    let mut epsilon_prime = 1.0 / pow_with_uint(10u, ii);

    // while we still need digits
    // build stack of digits
    while ii > 0u && (frac >= epsilon_prime || exact) {
        // store the next digit
        frac *= 10.0;
        let digit = frac as uint;
        vec::push(fractionalParts, digit);

        // calculate the next frac
        frac -= digit as float;
        epsilon_prime *= 10.0;
        ii -= 1u;
    }

    let mut acc;
    let mut racc = "";
    let mut carry = if frac * 10.0 as uint >= 5u { 1u } else { 0u };

    // turn digits into string
    // using stack of digits
    while vec::len(fractionalParts) > 0u {
         let mut adjusted_digit = carry + vec::pop(fractionalParts);

         if adjusted_digit == 10u { carry = 1u; adjusted_digit %= 10u } else { carry = 0u };

         racc = uint::str(adjusted_digit) + racc;
    }

    // pad decimals with trailing zeroes
    // given our precision problems this may be dead code
    while str::len(racc) < num_decimals && exact {
        racc += "0"
    }

    // combine ints and decimals
    let mut ones = uint::str(trunc + carry);
    if racc == "" {
       acc = ones;
    } else {
       acc = ones + "." + racc;
    }

    // pad leading ones
    while str::len(acc) < num_total {
        acc = " " + acc
    }

    acc = sign + acc;
   
    ret acc;
}

// originally from float.rs
fn my_to_str_exact(num: float, digits: uint) -> str {
    my_to_str_common(num, 0u, digits, true)
}

#[test]
fn rounding() {
   // trailing zeroes should be inserted, or rather
   // this dodgy floating point stuff should be rounded
   assert "3.141589999" == #fmt["%9.9f", 3.14159];
   assert "3.141589999" == float::to_str_common(3.14159, 9u, false);
   assert "3.141590000" == my_to_str_common(3.14159, 0u, 9u, false);

   assert "3.14158"     == float::to_str_common(3.14159, 5u, false);
   assert "3.14159"     == my_to_str_common(3.14159, 0u, 5u, false);
   
   // this truncation should be rounded
   assert "3.1415" == float::to_str(3.14159, 4u);
   assert "3.1416" == my_to_str_exact(3.14159, 4u);
   assert "3"      == my_to_str_exact(3.14159, 0u);
   assert "17"     == my_to_str_exact(16.9, 0u);
   assert "7.0000" == my_to_str_exact(6.99999999, 4u);

   // FIXME: epsilon calc should be adjusted and we should round these accordingly
   assert "3.14158999999999988261" == float::to_str_common(3.14159, 20u, false);
   assert "3.14158999999999988262" == my_to_str_exact(3.14159, 20u);
   assert "3.14158999999999988262" == my_to_str_common(3.14159, 0u, 20u, false);

   // total space padding
   assert "   3.141590000" == my_to_str_common(3.14159, 14u, 9u, false);
   assert "   3.141589999" == #fmt["%14.9f", 3.14159];
}


