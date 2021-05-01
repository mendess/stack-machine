use crate::make_test;

make_test!(rot3           : "1 2 3 @"                => @[2,3,1]);
make_test!(dup_rot3       : "1 2 3 _ @ ;"            => @[1,3,3]);
make_test!(copy           : "7 2 3 2 $"              => @[7,2,3,7]);
make_test!(rot            : "1 2 3 4 5 \\ ; @"       => @[1,3,5,2]);
make_test!(naosei         : "2 3 4 @ ; _ # \\ _ # +" => 283);
make_test!(hand_made_str  : "79 c 108 c 97 c"        => @['O','l','a']);
make_test!(hand_made_str2 : "79 108 97 c @ c @ c @"  => @['O','l','a']);
make_test!(stdin          : "l i l i #"              => 16              < "2\n4");
