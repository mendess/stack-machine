use crate::make_test;

make_test!(simple1     : "1 2 3 ?"                    => 2);
make_test!(simple2     : "0 2 3 ?"                    => 3);
make_test!(simple3     : "5 ,"                        => @[0,1,2,3,4]);
make_test!(simple4     : "5 , ~ \\"                   => "01243");
make_test!(simple5     : "[ 1 2 3 ] 2 * [ 4 5 ] \\ +" => [4,5,1,2,3,1,2,3]);
make_test!(wtf_is_this : "[ l l l ] { i 3 * } %"      => [3,6,9]                < "1\n2\n3");
make_test!(array       : "[ 3 1 9 ] ) 7 * + 3 *"      => [3,1,63,3,1,63,3,1,63]);
make_test!(planetas1   : "t N/ ~ #"                   => "3"                    < "planetas\nneta");
make_test!(planetas2   : "\"planetas\" 3 >"           => "tas");
make_test!(planetas3   : "\"planetas\" 3 <"           => "pla");
make_test!(planetas4   : "\"planetas\" 0 <"           => "");
make_test!(planetas5   : "\"planetas\" 0 >"           => "");
