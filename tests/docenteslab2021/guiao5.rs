use crate::make_test;

//make_test!(display_block : "2 { 3 * }"             => @[2, Rc::new()]);
make_test!(block_exec    : "2 { 3 * } ~"           => 6);
make_test!(transform     : "[ 1 2 3 ] { 2 # } %"   => [1,4,9]);
make_test!(caeser_cypher : "l { ( ( } %"           => "ola" < "qnc");
make_test!(incr_range    : "5 , { ) } %"           => [1,2,3,4,5]);
make_test!(filter_evens  : "5 , { 2 % } ,"         => [1, 3]);
make_test!(filter_mult_3 : "10 , { 3 % ! } ,"      => [0, 3, 6, 9]);
make_test!(two_blocks    : "10 , { ) } % { * } *"  => 3628800);
make_test!(max_numbers   : "t S/ { i } % { e> } *" => 13 < "2 7 13 4");
