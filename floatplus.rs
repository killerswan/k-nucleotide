use std;
import float::*;

//#[test]
fn summary() {
   assert "3.1416"      == #fmt["%.4f", 3.14159];
   assert "3"           == #fmt["%.0f", 3.14159];
   assert "99"          == #fmt["%.0f", 98.5];
   assert "7.0000"      == #fmt["%.4f", 6.999999999];
   assert "3.141590000" == #fmt["%.9f", 3.14159];
}

//#[test]
fn rounding() {
   // this truncation should be rounded
   assert "3.1416" == float::to_str(3.14159, 4u);
   assert "3.1416" == float::to_str_exact(3.14159, 4u);
   assert "3"      == float::to_str_exact(3.14159, 0u);
   assert "17"     == float::to_str_exact(16.9, 0u);
   assert "7.0000" == float::to_str_exact(6.99999999, 4u);
}

//#[test]
fn issue1876() {
   // #1876
   // trailing zeroes should be inserted, or rather
   // this dodgy floating point stuff should be rounded
   assert "3.141590000" == #fmt["%9.9f", 3.14159];
   assert "3.141590000" == float::to_str_common(3.14159, 9u, false);
   assert "3.14159"     == float::to_str_common(3.14159, 5u, false);
}
   
//#[test]
fn broken_issue1610() {
   // #1610
   // actually follow directions...
   assert "1.1000000000" /* FIXME */ == #fmt("%.100f", 1.1);
   assert "1.1000000000000000888178419700125232338905334472656250000000000000000000000000000000000000000000000000" == float::to_str_exact(1.1, 100u);
}

#[test]
fn issue1610_exploration() {
io::println(#fmt("%.2f", 1.1));
io::println(#fmt("%.50f", 1.1));
io::println(#fmt("%.75f", 1.1));
io::println(#fmt("%.90f", 1.1));
io::println(#fmt("%.99f", 1.1));
io::println(#fmt("%.100f", 1.1));
io::println("");

/* prints
   1.10
   1.10000000000000008881784197001252323389053344726563
   1.100000000000000088817841970012523233890533447265625000000000000000000000000
   1.100000000000000088817841970012523233890533447265625000000000000000000000000000000000000000
   1.100000000000000088817841970012523233890533447265625000000000000000000000000000000000000000000000000
   1.1000000000
*/
}

//#[test]
fn broken_precision() {
   // FIXME: epsilon calc should be adjusted and we should round these accordingly sometimes, right?
   assert "3.14158999999999988262" /* FIXME */ == float::to_str_exact(3.14159, 20u);
   assert "3.14158999999999988262" /* FIXME */ == float::to_str_common(3.14159, 20u, false);
}

// I believe the superficial parts of #1375 are covered now, but not the typeclass stuff



