# Stack Machine


## Expressões matemáticas

| done | ops         | example                                  |
| ---  | ---         | ---                                      |
| x    | + - * /     | Somar, subtrair, multiplicar e dividir   |
| x    | ( )         | Decrementar e incrementar um valor       |
| x    | %           | Módulo                                   |
| x    | #           | Exponenciação                            |
| x    | & &#x7c ^ ~ | E, ou, xor e not (bitwise) para inteiros |

## Stack

| done | ops         | example                                  |
| ---  | ---         | ---                                      |
|x| _   | Duplicar                                                        |
|x| ;   | Pop                                                             |
|x| \   | Trocar os dois elementos do topo da stack                       |
|x| @   | Rodar os 3 elementos no topo da stack                           |
|x| n $ | Copia n-ésimo elemento para o topo da stack 0 é o topo da stack |

## Lógica

| done | ops         | example                                  |
| ---  | ---         | ---                                      |
|x| 0 ou vazio     | False                                 |
|x| contrário de 0 | Verdadeiro                            |
|x| =              | Igual                                 |
|x| <              | Menor                                 |
|x| >              | Maior                                 |
|x| !              | Não                                   |
|x| e&             | E  (com shortcut)                     |
|x| e&#x7c         | Ou (com shortcut)                     |
|x| e<             | Coloca o menor dos 2 valores na stack |
|x| e>             | Coloca o maior dos 2 valores na stack |
|x| ?              | If-Then-Else                          |

## Variáveis

| done | ops         | example                                  |
| ---  | ---         | ---                                      |
|x| A até Z  | Coloca no topo da stack o conteúdo da variável |
|x| :<Letra> | Copia topo da stack à variável                 |
|x| A        | Valor por omissão: 10                          |
|x| B        | Valor por omissão: 11                          |
|x| C        | Valor por omissão: 12                          |
|x| D        | Valor por omissão: 13                          |
|x| E        | Valor por omissão: 14                          |
|x| F        | Valor por omissão: 15                          |
|x| N        | Valor por omissão: '\n'                        |
|x| S        | Valor por omissão: ' '                         |
|x| X        | Valor por omissão: 0                           |
|x| Y        | Valor por omissão: 1                           |
|x| Z        | Valor por omissão: 2                           |

## Input/Output

| done | ops         | example                                  |
| ---  | ---         | ---                                      |
|x| l | Ler linha              |
|x| t | Ler todas as linhas    |
|x| p | Imprimir topo da stack |

## Conversões

| done | ops         | example                                  |
| ---  | ---         | ---                                      |
|x| i | Converter o topo da stack num inteiro   |
|x| f | Converter o topo da stack num double    |
|x| c | Converter o topo da stack para caratere |
|x| s | Converter o topo da stack para string   |

## Arrays e strings

| done | ops         | example                                  |
| ---  | ---         | ---                                      |
|x| ""  | Criar uma string                                                        |
|x| []  | Criar um array                                                          |
|x| ~   | Colocar na stack todos os elementos do array                            |
|x| +   | Concatenar strings ou arrays                                            |
|x|     | (ou array/string com elemento)                                          |
|x| *   | Concatenar várias vezes strings ou arrays                               |
|x| ,   | Tamanho ou range                                                        |
|x| =   | Ir buscar um valor por índice                                           |
|x| < > | Ir buscar X elems/carat do início ou fim                                |
|x| ( ) | Remover 1º ou últ. elt. e colocar na stack após o array/string          |
|x| #   | Procurar substring na string e devolver o índice ou -1 se não encontrar |
|x| t   | Ler todo o input => String                                              |
|x| /   | Separar string por substring => Array                                   |
|x| S/  | Separar uma string por whitespace => Array                              |
|x| N/  | Separar uma string por newlines => Array                                |

## Blocos

| done | ops         | example                                  |
| ---  | ---         | ---                                      |
|x| {} | Criar um bloco                                                                             |
|x| ~  | Executar bloco                                                                             |
|x| %  | Aplicar o bloco a um array/string                                                          |
|x| *  | Fold sobre um array usando o bloco                                                         |
|x| ,  | Filtrar um array/string utilizando um bloco                                                |
| | $  | Ordenar usando o bloco                                                                     |
| | w  | Executa o bloco enquanto ele deixar um truthy no topo da stack; Remove da stack a condição |

# Exemplos

| Input                                      | Resultado    |
| ---                                        | ---          |
| 1 2 3 ?                                    | 2            |
| 0 2 3 ?                                    | 3            |
| 5 ,                                        | 01234        |
| 5 , ~ \                                    | 01243        |
| [ 1 2 3 ] 2 * [ 4 5 ] \ +                  | 45123123     |
| [ l l l ] { i 3 * } % 1 2 3                | 369          |
| [ 3 1 9 ] ) 7 * + 3 *                      | 316331633163 |
| t N/ ~ # planetas neta                     | 3            |
| "planetas" 3 >                             | tas          |
| 5 4 -                                      | 1            |
| 5 3 ) #                                    | 625          |
| 2 4 5 * +                                  | 22           |
| 1 ) ) 7 ( ( ( #                            | 81           |
| 5 2 %                                      | 1            |
| 2 5 ^                                      | 7            |
| 2 5 &                                      | 0            |
| 12 7 2 & &#x7c                             | 14           |
| 1 2 3 @                                    | 231          |
| 1 2 3 _ @ ;                                | 133          |
| 7 2 3 2 $                                  | 7237         |
| 1 2 3 4 5 \ ; @                            | 1352         |
| 2 3 4 @ ; _ # \ _ # +                      | 283          |
| 79 c 108 c 97 c                            | Ola          |
| 79 108 97 c @ c @ c @                      | Ola          |
| l i l i # 2 4                              | 16           |
| 7 2 3 ?                                    | 2            |
| 5 2 = 2 3 ?                                | 3            |
| 1 3 e&                                     | 3            |
| 0 3 e&                                     | 0            |
| 1 2 e&#x7c                                 | 1            |
| 0 2 e&#x7c                                 | 2            |
| 3 2 1 + =                                  | 1            |
| 3 5 = ! 7 2 >                              | 11           |
| 1 2 < 2 1 >                                | 11           |
| 3 5 2 e< e<                                | 2            |
| A B * :T T                                 | 110110       |
| 5 ,                                        | 01234        |
| [ 7 2 3 ] ,                                | 3            |
| "abc" 3 * _ S \ ,                          | abcabcabc 9  |
| 1 [ 2 3 ] + 3 *                            | 123123123    |
| [ 3 5 7 1 2 ] 2 =                          | 7            |
| [ 1 2 3 ] [ 4 5 ] \ +                      | 45123        |
| [ 7 2 9 ] (                                | 297          |
| 5 , 3 >                                    | 234          |
| [ 1 2 3 ] ( + [ 7 5 ] +                    | 23175        |
| [1 2 3] ~ * +                              | 7            |
| "olaqqabcqqxyz" "qq" / ,                   | 3            |
| t S/ , tres tristes tigres  barao vermelho | 5            |
| 2 { 3 * }                                  | 2{ 3 * }     |
| 2 { 3 * } ~                                | 6            |
| [ 1 2 3 ] { 2 # } %                        | 149          |
| l { ( ( } % qnc                            | ola          |
| 5 , { ) } %                                | 12345        |
| 5 , { 2 % } ,                              | 13           |
| 10 , { 3 % ! } ,                           | 0369         |
| 10 , { ) } % { * } *                       | 3628800      |
| t S/ { i } % { e> } * 2 7 13 4             | 13           |

