use crate::make_test;

make_test!(ternary  : "7 2 3 ?"       => 2);
make_test!(eq       : "5 2 = 2 3 ?"   => 3);
make_test!(and      : "1 3 e&"        => 3);
make_test!(and2     : "0 3 e&"        => 0);
make_test!(or       : "1 2 e|"        => 1);
make_test!(or2      : "0 2 e|"        => 2);
make_test!(arith_eq : "3 2 1 + ="     => 1);
make_test!(eq_diff  : "3 5 = ! 7 2 >" => @[1,1]);
make_test!(lt_gt    : "1 2 < 2 1 >"   => @[1,1]);
make_test!(ltlt     : "3 5 2 e< e<"   => 2);
make_test!(vars     : "A B * :T T"    => @[110,110]);

