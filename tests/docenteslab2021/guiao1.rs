use crate::make_test;

make_test!(sub       : "5 4 -"           => 1);
make_test!(incr_exp  : "5 3 ) #"         => 625);
make_test!(mul_add   : "2 4 5 * +"       => 22);
make_test!(incr_decr : "1 ) ) 7 ( ( ( #" => 81);
make_test!(modulo    : "5 2 %"           => 1);
make_test!(xor       : "2 5 ^"           => 7);
make_test!(and       : "2 5 &"           => 0);
make_test!(and_or    : "12 7 2 & |"      => 14);
