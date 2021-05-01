use crate::make_test;

make_test!(range_num  : "5 ,"                         => [0,1,2,3,4]);
make_test!(array      : "[ 7 2 3 ] ,"                 => 3);
make_test!(string_mul : r#""abc" 3 * _ S \ ,"#        => @["abcabcabc", ' ', 9]);
make_test!(array_add  : "1 [ 2 3 ] + 3 *"             => [1,2,3,1,2,3,1,2,3]);
make_test!(index      : "[ 3 5 7 1 2 ] 2 ="           => 7);
make_test!(concat     : "[ 1 2 3 ] [ 4 5 ] \\ +"      => [4, 5, 1, 2, 3]);
make_test!(rm_head    : "[ 7 2 9 ] ("                 => @[[2, 9], 7]);
make_test!(slice      : "5 , 3 >"                     => [2, 3, 4]);
make_test!(rm_concat  : " [ 1 2 3 ] ( + [ 7 5 ] +"    => [2, 3, 1, 7, 5]);
make_test!(flat_math  : " [ 1 2 3 ] ~ * +"            => 7);
make_test!(split      : r#""olaqqabcqqxyz" "qq" / ,"# => 3);
make_test!(tokenize   : "t S/ ,"                      => 5 < "tres tristes tigres\nbarao vermelho");
