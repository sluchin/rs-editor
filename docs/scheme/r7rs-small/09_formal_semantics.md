# 形式的意味論

560ZZ

本節は， Scheme の原始式といくつか選んだ組込み手続きに対する形式的な 表示的意味論を定める。 ここで使う概念と表記は に記述されている。 `dynamic-wind` の定義は  から得た。 表記を下記に要約する:

|                                    |                                                       |
|:-----------------------------------|:------------------------------------------------------|
| $`\langle\,\ldots\,\rangle`$       | 列の形成                                              |
| $`s \downarrow k`$                 | 列$`s`$の第$`k`$要素 (1から数えて)                    |
| $`\#s`$                            | 列$`s`$の長さ                                         |
| $`s \:\S\: t`$                     | 列$`s`$と列$`t`$の連結                                |
| $`s \dagger k`$                    | 列$`s`$の最初の$`k`$個の要素を落とす                  |
| $`t \rightarrow a, b`$             | McCarthy の条件式 \`\`if $`t`$ then $`a`$ else $`b`$” |
| $`\rho[x/i]`$                      | 置換 \`\`$`i`$の代わりに$`x`$を持った$`\rho`$”        |
| $`x\hbox\textrm{in }{\texttt{}D}`$ | 定義域$`{\texttt{}D}`$の中への$`x`$の単射             |
| $`x\,\vert\,{\texttt{}D}`$         | 定義域$`{\texttt{}D}`$への$`x`$の射影                 |

式の継続が，単一の値ではなく (複数個の) 値からなる列を受け取る理由は， 手続き呼出しと多重個の戻り値の形式的な取扱いを単純化するためである。

ペア，ベクタ，および文字列に結合したブーリアンのフラグは， 書換え可能オブジェクトならば真に，書換え不可能オブジェクトならば偽になる。

一つの呼出しにおける評価の順序は未規定である。 我々はここで，呼出しにおける引数を評価する前と評価した後に それらの引数に恣意的な順列並べ替え関数 *permute* と その逆関数 *unpermute* を適用することによってそれを模倣する。 これは評価の順序が (任意の数の引数について) プログラム全体で一定であると間違って示唆しているので まったく適切ではないが，それは左から右への評価よりも意図された意味論に近い近似である。

記憶領域を割り当てる関数 *new* は実装依存だが，次の公理に従わなければ ならない: もし なら ば $`\sigma\:(\hbox\textit{new}\:\sigma\:\vert\:`L`)\downarrow 2 = \textit{false}`$.

$`\hbox{$\cal K$}`$ の定義は省略する。なぜなら $`\hbox{$\cal K$}`$ の精密な定義は， あまりおもしろみなく意味論を複雑化するだろうからである。

もしも <span class="roman">P</span> がプログラムであって，そのすべての変数が参照または代入される前に 定義されているならば，<span class="roman">P</span> の意味は
``` math
\hbox{$\cal E$}[\![\hbox{\texttt{((lambda (\hbox\textrm{I}*) \hbox\textrm{P}')
\textit{⟨未規定⟩} …)}}]\!]
```
である。ここで <span class="roman">I</span>\* は <span class="roman">P</span> で定義されている変数の列であり， $`\hbox\textrm{P}'`$ は <span class="roman">P</span> の中の定義をそれぞれ代入で置き換えることによって 得られる式の列であり， *⟨未規定⟩* は *undefined* (未定義値) へと評価される式であり， そして $`\hbox{$\cal E$}`$ は意味を式に割り当てる意味関数 (semantic function) である。

## 抽象構文

|  |  |  |  |
|---:|:--:|:---|:---|
| <span class="roman">K</span> |  | <span class="roman">Con</span> | 引用を含む定数 |
| <span class="roman">I</span> |  | <span class="roman">Ide</span> | 識別子 (変数) |
| <span class="roman">E</span> |  | <span class="roman">Exp</span> | 式 |
|  |  | <span class="roman">Com</span> $`=`$ <span class="roman">Exp</span> | コマンド |

```
\Exp → \K | \I | (\E_0 \E)
 | (lambda (\I) \C \E_0)
 | (lambda (\I . \I) \C \E_0)
 | (lambda \I \C \E_0)
 | (if \E_0 \E_1 \E_2) | (if \E_0 \E_1)
 | (set! \I \E)
```

## ドメイン等式

|  |  |  |  |  |  |
|---:|:--:|:---|:---|:---|:---|
| $`\alpha`$ |  | `L` |  |  | 場所 (locations) |
| $`\nu`$ |  | `N` |  |  | 自然数 |
|  |  | `T` | = | $`\{`$*false, true$`\}`$* | ブーリアン |
|  |  | `Q` |  |  | シンボル |
|  |  | `H` |  |  | 文字 |
|  |  | `R` |  |  | 数 |
|  |  |  | = | $``L`\times `L`\times `T``$ | ペア |
|  |  |  | = | $``L`* \times `T``$ | ベクタ |
|  |  |  | = | $``L`* \times `T``$ | 文字列 |
|  |  | `M` | = |  |  |
|  |  |  |  |  | 雑値 (miscellaneous) |
| $`\phi`$ |  | `F` | = | $``L`\times(`E`* \to \DP \to `K`\to `C`)`$ | 手続き値 |
| $`\epsilon`$ |  | `E` | = |  |  |
|  |  |  |  |  | 式の値 |
| $`\sigma`$ |  | `S` | = | $``L`\to(`E`\times `T`)`$ | 記憶装置 (stores) |
| $`\rho`$ |  | `U` | = | $`\hbox\textrm{Ide}\to `L``$ | 環境 |
| $`\theta`$ |  | `C` | = | $``S`\to `A``$ | 式の継続 |
| $`\kappa`$ |  | `K` | = | $``E`*\to `C``$ | 式の継続 |
|  |  | `A` |  |  | 答え (answers) |
|  |  | `X` |  |  | エラー |
| $`\omega`$ |  |  | = | $`(`F`\times `F`\times \DP) + \{\textit{root}\}`$ | 動的ポイント |

## 意味関数

|                       |                                                                                 |
|----------------------:|:--------------------------------------------------------------------------------|
|  $`\hbox{$\cal K$}:`$ | $`\hbox\textrm{Con}\to `E``$                                          |
|  $`\hbox{$\cal E$}:`$ | $`\hbox\textrm{Exp}\to `U`\to\DP\to `K`\to `C``$  |
| $`\hbox{$\cal E$}*:`$ | $`\hbox\textrm{Exp}*\to `U`\to\DP\to `K`\to `C``$ |
|  $`\hbox{$\cal C$}:`$ | $`\hbox\textrm{Com}*\to `U`\to\DP\to `C`\to `C``$ |

ここで の定義は故意に省略する。

```latex
\Esem\sembrack{\K} =
  \lambda\rho\omega\kappa\:.\:\fun{send}\,(\Ksem\sembrack{\K})\,\kappa
```

```latex
\Esem\sembrack{\I} = 
  \lambda\rho\omega\kappa\:.\:\fun{hold}\:
    $\=$(\fun{lookup}\:\rho\:\I)$\\
     \>$(\fun{single}(\lambda\epsilon\:.\:
        $\=$\epsilon = \fun{undefined}\rightarrow$\\
     \>  \> \go{2}$\wrong{未定義変数},$\\
     \>  \>\go{1}$\fun{send}\:\epsilon\:\kappa))
```

```latex
\Esem\sembrack{\hbox{\texttt{($\E_0$ \arbno{\E})}}} =$\\
 \go{1}$\lambda\rho\omega\kappa\:.\:\arbno{\Esem}
    $\=$(\fun{permute}(\langle\E_0\rangle\:\S\:\arbno{\E}))$\\
     \>$\rho\:$\\
     \>$\omega\:$\\
     \>$(\lambda\arbno{\epsilon}\:.\:
        ($\=$(\lambda\arbno{\epsilon}\:.\:
                 \fun{applicate}\:(\arbno{\epsilon}\elt 1)
                                \:(\arbno{\epsilon}\drop 1)
                                \:\omega\kappa)$\\
     \>   \>$(\fun{unpermute}\:\arbno{\epsilon})))
```

```latex
\Esem\sembrack{\hbox{\texttt{(\ide{lambda} (\arbno{\I}) \arbno{\C} $\E_0$)}}} =$\\
 \go{1}$\lambda\rho\omega\kappa\:.\:\lambda\sigma\:.\:$\\
  \go{2}$\fun{new}\:\sigma\:\elem\:\LOC\rightarrow$\\
   \go{3}$\fun{send}\:
     $\=$(\langle
         $\=$\fun{new}\:\sigma\,\vert\,\LOC,$\\
      \>  \>$\lambda\arbno{\epsilon}\omega^\prime\kappa^\prime\:.\:
               $\=$\#\arbno{\epsilon} = \#{\arbno{\I}}\rightarrow$\\
      \>  \>    $\go{1}\fun{tievals}
                   $\=$(\lambda\arbno{\alpha}\:.\:
                         $\=$(\lambda\rho^\prime\:.\:\Csem\sembrack{\arbno{\C}}\rho^\prime\omega^\prime
                              (\Esem\sembrack{\E_0}\rho^\prime\omega^\prime\kappa^\prime))$\\
      \>  \>      \>    \>$(\fun{extends}\:\rho\:{\arbno{\I}}\:\arbno{\alpha}))$\\
      \>  \>      \>$\arbno{\epsilon},$\\
      \>  \>    \go{1}$\wrong{引数の個数違い}\rangle$\\
      \>  \>$\hbox{ \rm in }\EXP)$\\
      \>$\kappa$\\
      \>$(\fun{update}\:(\fun{new}\:\sigma\,\vert\,\LOC)
                           \:\fun{unspecified}
                           \:\sigma),$\\
  \go{3}$\wrong{メモリ不足}\:\sigma
```

```latex
\Esem\sembrack{\hbox{\texttt{(lambda (\arbno{\I} {\bf.}\ \I) \arbno{\C} $\E_0$)}}} =$\\
 \go{1}$\lambda\rho\omega\kappa\:.\:\lambda\sigma\:.\:$\\
  \go{2}$\fun{new}\:\sigma\:\elem\:\LOC\rightarrow$\\
   \go{3}$\fun{send}\:
     $\=$(\langle
         $\=$\fun{new}\:\sigma\,\vert\,\LOC,$\\
      \>  \>$\lambda\arbno{\epsilon}\omega^\prime\kappa^\prime\:.\:
               $\=$\#\arbno{\epsilon} \geq \#\arbno{\I}\rightarrow$\\
      \>  \>    \>\go{1}$\fun{tievalsrest}$\\
      \>  \>    \>\go{2}\=$(\lambda\arbno{\alpha}\:.\:
                           $\=$(\lambda\rho^\prime\:.\:\Csem\sembrack{\arbno{\C}}\rho^\prime\omega^\prime
                               (\Esem\sembrack{\E_0}\rho^\prime\omega^\prime\kappa^\prime))$\\
      \>  \>    \>       \> \>$(\fun{extends}\:\rho
                               \:(\arbno{\I}\:\S\:\langle\I\rangle)
                               \:\arbno{\alpha}))$\\
      \>  \>    \>       \>$\arbno{\epsilon}$\\
      \>  \>    \>       \>$(\#\arbno{\I}),$\\
      \>  \>    \>\go{1}$\wrong{引数が少な過ぎる}\rangle\hbox{ \rm in }\EXP)$\\
      \>$\kappa$\\
      \>$(\fun{update}\:(\fun{new}\:\sigma\,\vert\,\LOC)
                           \:\fun{unspecified}
                           \:\sigma),$\\
  \go{3}$\wrong{メモリ不足}\:\sigma
```

```latex
\Esem\sembrack{\hbox{\texttt{(lambda \I{} \arbno{\C} $\E_0$)}}} =
 \Esem\sembrack{\hbox{\texttt{(lambda ({\bf.}\ \I) \arbno{\C} $\E_0$)}}}
```

```latex
\Esem\sembrack{\hbox{\texttt{(\ide{if} $\E_0$ $\E_1$ $\E_2$)}}} =$\\
 \go{1}$\lambda\rho\omega\kappa\:.\:
   \Esem\sembrack{\E_0}\:\rho\omega\:(\fun{single}\:(\lambda\epsilon\:.\:
    $\=$\fun{truish}\:\epsilon\rightarrow\Esem\sembrack{\E_1}\rho\omega\kappa,$\\
     \>\go{1}$\Esem\sembrack{\E_2}\rho\omega\kappa))
```

```latex
\Esem\sembrack{\hbox{\texttt{(if $\E_0$ $\E_1$)}}} =$\\
 \go{1}$\lambda\rho\omega\kappa\:.\:
   \Esem\sembrack{\E_0}\:\rho\omega\:(\fun{single}\:(\lambda\epsilon\:.\:
    $\=$\fun{truish}\:\epsilon\rightarrow\Esem\sembrack{\E_1}\rho\omega\kappa,$\\
     \>\go{1}$\fun{send}\:\fun{unspecified}\:\kappa))
```

ここでも他のところでも，*undefined* (未定義値) を除く任意の式の値 を，*unspecified* (未規定値) のところに使ってよい。

```latex
\Esem\sembrack{\hbox{\texttt{(\ide{set!} \I{} \E)}}} =$\\
 \go{1}$\lambda\rho\omega\kappa\:.\:\Esem\sembrack{\E}\:\rho\:\omega\:
     (\fun{single}(\lambda\epsilon\:.\:\fun{assign}\:
       $\=$(\fun{lookup}\:\rho\:\I)$\\
        \>$\epsilon$\\
        \>$(\fun{send}\:\fun{unspecified}\:\kappa)))
```

```latex
\arbno{\Esem}\sembrack{\:} =
  \lambda\rho\omega\kappa\:.\:\kappa\langle\:\rangle
```

```latex
\arbno{\Esem}\sembrack{\E_0\:\arbno{\E}} =$\\
 \go{1}$\lambda\rho\omega\kappa\:.\:
      \Esem\sembrack{\E_0}\:\rho\omega\:
         (\fun{single}
            (\lambda\epsilon_0\:.\:\arbno{\Esem}\sembrack{\arbno{\E}}
                \:\rho\omega\:(\lambda\arbno{\epsilon}\:.\:
                           \kappa\:(\langle\epsilon_0\rangle\:\S\:\arbno{\epsilon}))))
```

```latex
\Csem\sembrack{\:} = \lambda\rho\omega\theta\,.\:\theta
```

```latex
\Csem\sembrack{\C_0\:\arbno{\C}} =
  \lambda\rho\omega\theta\:.\:\Esem\sembrack{\C_0}\:\rho\omega\:(\lambda\arbno{\epsilon}\:.\:
   \Csem\sembrack{\arbno{\C}}\rho\omega\theta)
```

## 補助関数

```latex
\fun{lookup}        :  \ENV \to \Ide \to \LOC$\\$
\fun{lookup} =
 \lambda\rho\I\:.\:\rho\I
```

```latex
\fun{extends}       :  \ENV \to \arbno{\Ide} \to \arbno{\LOC} \to \ENV$\\$
\fun{extends} =$\\
 \go{1}$\lambda\rho\arbno{\I}\arbno{\alpha}\:.\:
   $\=$\#\arbno{\I}=0\rightarrow\rho,$\\
    \>$\go{1}\fun{extends}\:(\rho[(\arbno{\alpha}\elt 1)/(\arbno{\I}\elt 1)])
                               \:(\arbno{\I}\drop 1)
                               \:(\arbno{\alpha}\drop 1)
```

```latex
\fun{wrong}  :  \ERR \to \CC    \hbox{\qquad [実装依存]}
```

```latex
\fun{send}          :  \EXP \to \EC \to \CC$\\$
\fun{send} =
 \lambda\epsilon\kappa\:.\:\kappa\langle\epsilon\rangle
```

```latex
\fun{single}        :  (\EXP \to \CC) \to \EC$\\$
\fun{single} =$\\
 \go{1}$\lambda\psi\arbno{\epsilon}\:.\:
   $\=$\#\arbno{\epsilon}=1\rightarrow\psi(\arbno{\epsilon}\elt 1),$\\
    \>$\go{1}\wrong{戻り値の個数違い}
```

```latex
\fun{new}           :  \STO \to (\LOC + \{ \fun{error} \})
    \hbox{\qquad [実装依存]}
```

```latex
\fun{hold}          :  \LOC \to \EC \to \CC$\\$
\fun{hold} =
 \lambda\alpha\kappa\sigma\:.\:\fun{send}\,(\sigma\alpha\elt 1)\kappa\sigma
```

```latex
\fun{assign}        :  \LOC \to \EXP \to \CC \to \CC$\\$
\fun{assign} =
 \lambda\alpha\epsilon\theta\sigma\:.\:\theta(\fun{update}\:\alpha\epsilon\sigma)
```

```latex
\fun{update}        :  \LOC \to \EXP \to \STO \to \STO$\\$
\fun{update} =
 \lambda\alpha\epsilon\sigma\:.\:\sigma[\langle\epsilon,\fun{true}\rangle/\alpha]
```

```latex
\fun{tievals}       :  (\arbno{\LOC} \to \CC) \to \arbno{\EXP} \to \CC$\\$
\fun{tievals} =$\\
 \go{1}$\lambda\psi\arbno{\epsilon}\sigma\:.\:
   $\=$\#\arbno{\epsilon}=0\rightarrow\psi\langle\:\rangle\sigma,$\\
    \>$\fun{new}\:\sigma\:\elem\:\LOC\rightarrow\fun{tievals}\,
       $\=$(\lambda\arbno{\alpha}\:.\:\psi(\langle\fun{new}\:\sigma\:\vert\:\LOC\rangle
                                     \:\S\:\arbno{\alpha}))$\\
    \>  \>$(\arbno{\epsilon}\drop 1)$\\
    \>  \>$(\fun{update}(\fun{new}\:\sigma\:\vert\:\LOC)
                                 (\arbno{\epsilon}\elt 1)
                                 \sigma),$\\
    \>$\go{1}\wrong{メモリ不足}\sigma
```

```latex
\fun{tievalsrest}   :  (\arbno{\LOC} \to \CC) \to \arbno{\EXP} \to \NAT \to \CC$\\$
\fun{tievalsrest} =$\\
 \go{1}$\lambda\psi\arbno{\epsilon}\nu\:.\:\fun{list}\:
   $\=$(\fun{dropfirst}\:\arbno{\epsilon}\nu)$\\
    \>$(\fun{single}(\lambda\epsilon\:.\:\fun{tievals}\:\psi\:
           ((\fun{takefirst}\:\arbno{\epsilon}\nu)\:\S\:\langle\epsilon\rangle)))
```

```latex
\fun{dropfirst} =
 \lambda l n \:.\:  n=0 \rightarrow l, \fun{dropfirst}\,(l \drop 1)(n - 1)
```

```latex
\fun{takefirst} =
 \lambda l n \:.\: n=0 \rightarrow \langle\:\rangle,
     \langle l \elt 1\rangle\:\S\:(\fun{takefirst}\,(l \drop 1)(n - 1))
```

```latex
\fun{truish}        :  \EXP \to \TRU$\\$
\fun{truish} =
  \lambda\epsilon\:.\:

     \epsilon = \fun{false}\rightarrow
          \fun{false},
          \fun{true}
```

```latex
\fun{permute}       :  \arbno{\Exp} \to \arbno{\Exp}
    \hbox{\qquad [実装依存]}
```

```latex
\fun{unpermute}     :  \arbno{\EXP} \to \arbno{\EXP}
    \hbox{\qquad [\fun{permute} の逆関数]}
```

```latex
\fun{applicate}     :  \EXP \to \arbno{\EXP} \to \DP \to \EC \to \CC$\\$
\fun{applicate} =$\\
 \go{1}$\lambda\epsilon\arbno{\epsilon}\omega\kappa\:.\:
   $\=$\epsilon\:\elem\:\FUN\rightarrow(\epsilon\:\vert\:\FUN\elt 2)\arbno{\epsilon}\omega\kappa,
          \wrong{無効手続き}
```

```latex
\fun{onearg}      :  (\EXP \to \DP \to \EC \to \CC) \to (\arbno{\EXP} \to \DP \to \EC \to \CC)$\\$
\fun{onearg} =$\\
 \go{1}$\lambda\zeta\arbno{\epsilon}\omega\kappa\:.\:
   $\=$\#\arbno{\epsilon}=1\rightarrow\zeta(\arbno{\epsilon}\elt 1)\omega\kappa,$\\
    \>$\go{1}\wrong{引数の個数違い}
```

```latex
\fun{twoarg}      :  (\EXP \to \EXP \to \DP \to \EC \to \CC) \to (\arbno{\EXP} \to \DP \to \EC \to \CC)$\\$
\fun{twoarg} =$\\
 \go{1}$\lambda\zeta\arbno{\epsilon}\omega\kappa\:.\:
   $\=$\#\arbno{\epsilon}=2\rightarrow\zeta(\arbno{\epsilon}\elt 1)(\arbno{\epsilon}\elt 2)\omega\kappa,$\\
    \>$\go{1}\wrong{引数の個数違い}
```

```latex
\fun{threearg}      :  (\EXP \to \EXP \to \EXP \to \DP \to \EC \to \CC) \to (\arbno{\EXP} \to \DP \to \EC \to \CC)$\\$
\fun{threearg} =$\\
 \go{1}$\lambda\zeta\arbno{\epsilon}\omega\kappa\:.\:
   $\=$\#\arbno{\epsilon}=3\rightarrow\zeta(\arbno{\epsilon}\elt 1)(\arbno{\epsilon}\elt 2)(\arbno{\epsilon}\elt 3)\omega\kappa,$\\
    \>$\go{1}\wrong{引数の個数違い}
```

```latex
\fun{list}          :  \arbno{\EXP} \to \DP \to \EC \to \CC$\\$
\fun{list} =$\\
 \go{1}$\lambda\arbno{\epsilon}\omega\kappa\:.\:
   $\=$\#\arbno{\epsilon}=0\rightarrow\fun{send}\:\fun{null}\:\kappa,$\\
    \>$\go{1}\fun{list}\,(\arbno{\epsilon}\drop 1)
             (\fun{single}(\lambda\epsilon\:.\:
                   \fun{cons}\langle\arbno{\epsilon}\elt 1,\epsilon\rangle\kappa))
```

```latex
\fun{cons}          :  \arbno{\EXP} \to \DP \to \EC \to \CC$\\$
\fun{cons} =$\\
 \go{1}$\fun{twoarg}\,(\lambda\epsilon_1\epsilon_2\kappa\omega\sigma\:.\:
   $\=$\fun{new}\:\sigma\:\elem\:\LOC\rightarrow$\\
    \> 
        \=$(\lambda\sigma^\prime\:.\:
           $\=$\fun{new}\:\sigma^\prime\:\elem\:\LOC\rightarrow$\\
    \>  \>$\go{1}\fun{send}\,
               $\=$($\=$\langle\fun{new}\:\sigma\:\vert\:\LOC,
                                            \fun{new}\:\sigma^\prime\:\vert\:\LOC,
         \fun{true}\rangle$\\
                                \>  \>  \>  \>$\hbox{ \rm in }\EXP)$\\
    \>  \>  \>$\kappa$\\
    \>  \>  \>$(\fun{update}(\fun{new}\:\sigma^\prime\:\vert\:\LOC)
                                     \epsilon_2
                                     \sigma^\prime),$\\
    \>  \>$\go{1}\wrong{メモリ不足}\sigma^\prime)$\\
    \>  $(\fun{update}(\fun{new}\:\sigma\:\vert\:\LOC)\epsilon_1\sigma),$\\
    \>$\wrong{メモリ不足}\sigma)
```

```latex
\fun{less}          :  \arbno{\EXP} \to \DP \to \EC \to \CC$\\$
\fun{less} =$\\
 \go{1}$\fun{twoarg}\,(\lambda\epsilon_1\epsilon_2\omega\kappa\:.\:
   $\=$(\epsilon_1\:\elem\:\NUM\wedge\epsilon_2\:\elem\:\NUM)\rightarrow$\\
    \>$\go{1}\fun{send}\,
               (\epsilon_1\:\vert\:\NUM<\epsilon_2\:\vert\:\NUM\rightarrow
                   \fun{true},
                   \fun{false})
               \kappa,$\\
    \>$\go{1}\wrong{{\cf <} に非数値引数})
```

```latex
\fun{add}          :  \arbno{\EXP} \to \DP \to \EC \to \CC$\\$
\fun{add} =$\\
 \go{1}$\fun{twoarg}\,(\lambda\epsilon_1\epsilon_2\omega\kappa\:.\:
   $\=$(\epsilon_1\:\elem\:\NUM\wedge\epsilon_2\:\elem\:\NUM)\rightarrow$\\
    \>$\go{1}\fun{send}\,
       $\=$((\epsilon_1\:\vert\:\NUM+\epsilon_2\:\vert\:\NUM)\hbox{ \rm in }\EXP)
           \kappa,$\\
    \>$\go{1}\wrong{{\cf <} に非数値引数})
```

```latex
\fun{car}          :  \arbno{\EXP} \to \DP \to \EC \to \CC$\\$
\fun{car} =$\\
 \go{1}$\fun{onearg}\,(\lambda\epsilon\omega\kappa\:.\:
   $\=$\epsilon\:\elem\:\PAI\rightarrow
          \fun{car-internal}\:\epsilon\kappa,$\\
    \>$\go{1}\wrong{{\cf car} に非ペア引数})
```

```latex
\fun{car-internal}          :  \EXP \to \EC \to \CC$\\$
\fun{car-internal} =
 $\go{1}$\lambda\epsilon\omega\kappa\:.\:
   $\=$\fun{hold}\, (\epsilon\:\vert\:\PAI\elt 1) \kappa
```

```latex
\fun{cdr}          :  \arbno{\EXP} \to \DP \to \EC \to \CC 
\hbox{\qquad [\fun{car} と同様]}
```

```latex
\fun{cdr-internal} :  \EXP \to \EC \to \CC 
\hbox{\qquad [\fun{car-internal} と同様]}
```

```latex
\fun{setcar}          :  \arbno{\EXP} \to \DP \to \EC \to \CC$\\$
\fun{setcar} =$\\
 \go{1}$\fun{twoarg}\,(\lambda\epsilon_1\epsilon_2\omega\kappa\:.\:
   $\=$\epsilon_1\:\elem\:\PAI\rightarrow$\\
    \>$(\epsilon_1\:\vert\:\PAI\elt 3)\rightarrow
          \fun{assign}\,$\=$(\epsilon_1\:\vert\:\PAI\elt 1)$\\
    \>                           \>$\epsilon_2$\\
    \>                                  \>$(\fun{send}\:\fun{unspecified}\:\kappa),$\\
    \>$\wrong{{\cf set-car!} に書換え不可能引数},$\\
    \>$\wrong{{\cf set-car!} に非ペア引数})
```

```latex
\fun{eqv}          :  \arbno{\EXP} \to \DP \to \EC \to \CC$\\$
\fun{eqv} =$\\
 \go{1}$\fun{twoarg}\,(\lambda\epsilon_1\epsilon_2\omega\kappa\:.\:
   $\=$(\epsilon_1\:\elem\:\MSC\wedge\epsilon_2\:\elem\:\MSC)\rightarrow$\\
    \>$\go{1}\fun{send}\,
       $\=$(\epsilon_1\:\vert\:\MSC = \epsilon_2\:\vert\:\MSC\rightarrow\fun{true},
            \fun{false})\kappa,$\\
    \>$(\epsilon_1\:\elem\:\SYM\wedge\epsilon_2\:\elem\:\SYM)\rightarrow$\\
    \>$\go{1}\fun{send}\,
       $\=$(\epsilon_1\:\vert\:\SYM = \epsilon_2\:\vert\:\SYM\rightarrow\fun{true},
            \fun{false})\kappa,$\\
    \>$(\epsilon_1\:\elem\:\CHR\wedge\epsilon_2\:\elem\:\CHR)\rightarrow$\\
    \>$\go{1}\fun{send}\,
       $\=$(\epsilon_1\:\vert\:\CHR = \epsilon_2\:\vert\:\CHR \rightarrow\fun{true},
            \fun{false})\kappa,$\\
    \>$(\epsilon_1\:\elem\:\NUM\wedge\epsilon_2\:\elem\:\NUM)\rightarrow$\\
    \>$\go{1}\fun{send}\,
       $\=$(\epsilon_1\:\vert\:\NUM=\epsilon_2\:\vert\:\NUM\rightarrow\fun{true},
            \fun{false})\kappa,$\\
    \>$(\epsilon_1\:\elem\:\PAI\wedge\epsilon_2\:\elem\:\PAI)\rightarrow$\\
    \>$\go{1}\fun{send}\,
       $\=$($\=$(\lambda{p_1}{p_2}\:.\:
                ($\=$({p_1}\elt 1) = ({p_2}\elt 1)\wedge$\\
    \>  \>   \>   \>$({p_1}\elt 2) = ({p_2}\elt 2))
                     \rightarrow\fun{true},$\\
    \>  \>   \>   \>$\go{1}\fun{false})$\\
    \>  \>   \>$(\epsilon_1\:\vert\:\PAI)$\\
    \>  \>   \>$(\epsilon_2\:\vert\:\PAI))$\\
    \>  \>$\kappa,$\\
    \>$(\epsilon_1\:\elem\:\VEC\wedge\epsilon_2\:\elem\:\VEC)\rightarrow

\ldots,$\\
    \>$(\epsilon_1\:\elem\:\STR\wedge\epsilon_2\:\elem\:\STR)\rightarrow

\ldots,$\\
    \>$(\epsilon_1\:\elem\:\FUN\wedge\epsilon_2\:\elem\:\FUN)\rightarrow$\\
    \>$\go{1}\fun{send}\,
       $\=$((\epsilon_1\:\vert\:\FUN\elt 1) = (\epsilon_2\:\vert\:\FUN\elt 1)
               \rightarrow\fun{true},
                          \fun{false})$\\
    \>  \>$\kappa,$\\
    \>$\go{1}\fun{send}\,\:\fun{false}\:\kappa)
```

```latex
\fun{apply}          :  \arbno{\EXP} \to \DP \to \EC \to \CC$\\$
\fun{apply} =$\\
 \go{1}$\fun{twoarg}\,(\lambda\epsilon_1\epsilon_2\omega\kappa\:.\:
   $\=$\epsilon_1\:\elem\:\FUN\rightarrow
         \fun{valueslist}\:\epsilon_2
            (\lambda\arbno{\epsilon}\:.\:\fun{applicate}\:\epsilon_1\arbno{\epsilon}\omega\kappa),$\\
    \>$\go{1}\wrong{{\cf apply} に無効手続き引数})
```

```latex
\fun{valueslist}          :  \EXP \to \EC \to \CC$\\$
\fun{valueslist} =$\\
 \go{1}$\lambda\epsilon\kappa\:.\:
   $\=$\epsilon\:\elem\:\PAI\rightarrow$\\
    \>$\go{1}\fun{cdr-internal}\:
         $\=$\epsilon$\\
    \>    \>$(\lambda\arbno{\epsilon}\:.\:
                  $\=$\fun{valueslist}\:$\\
    \>    \>       \>$\arbno{\epsilon}$\\
    \>    \>       \>$(\lambda\arbno{\epsilon}\:.\:$\=$\fun{car-internal}$\\
    \>    \>       \>                               \>$\:\epsilon$\\
    \>    \>       \>                               \>$ (\fun{single}(\lambda\epsilon\:.\:
              \kappa(\langle\epsilon\rangle\:\S\:\arbno{\epsilon}))))),$\\
    \>$\epsilon = \fun{null}\rightarrow\kappa\langle\:\rangle,$\\
    \>$\go{1}\wrong{{\cf values-list} に非リスト引数}
```

```latex
\fun{cwcc}          $\=$:  \arbno{\EXP} \to \DP \to \EC \to \CC$\\$
    $\>$ \hbox{\qquad [\ide{call-with-current-continuation}]}$\\$
\fun{cwcc} =$\\
 \go{1}$\fun{onearg}\,(\lambda\epsilon\omega\kappa\:.\:
   $\=$\epsilon\:\elem\:\FUN\rightarrow$\\
    \>$(\lambda\sigma\:.\:
       $\=$\fun{new}\:\sigma\:\elem\:\LOC\rightarrow$\\
    \>  \>$\go{1}\fun{applicate}\:
           $\=$\epsilon$\\
    \>  \>  \>$\langle\langle$\=$\fun{new}\:\sigma\:\vert\:\LOC,$\\
    \>  \>  \>  \>$          \lambda\arbno{\epsilon}\omega^\prime\kappa^\prime\:.\:
                             \fun{travel}\:\omega^\prime\omega(\kappa\arbno{\epsilon})\rangle$\\
    \>  \>  \>$                      \hbox{ \rm in }\EXP\rangle$\\
    \>  \>  \>$\omega$\\
    \>  \>  \>$\kappa$\\
    \>  \>  \>$(\fun{update}\,
                $\=$(\fun{new}\:\sigma\:\vert\:\LOC)$\\
    \>  \>  \>   \>$\fun{unspecified}$\\
    \>  \>  \>   \>$\sigma),$\\
    \>  \>$\go{1}\wrong{メモリ不足}\,\sigma),$\\
    \>$\wrong{無効手続き引数})
```

```latex
\fun{travel} : \DP \to \DP \to \CC \to \CC$\\$
\fun{travel} = $\\
  \go{1}$\lambda\omega_1\omega_2\:.\:
  \fun{travelpath}\:($\=$(\fun{pathup}\:\omega_1(\fun{commonancest}\:\omega_1\omega_2)) \:\S\:$\\
  \>$ (\fun{pathdown}\:(\fun{commonancest}\:\omega_1\omega_2)\omega_2))
```

```latex
\fun{pointdepth} : \DP \to \NAT$\\$
\fun{pointdepth} = $\\
  \go{1}$\lambda\omega\:.\: \omega = \textit{root} \rightarrow 0,
  1 + (\fun{pointdepth}\:(\omega\:\vert\:(\FUN \times \FUN \times
  \DP)\elt 3))
```

```latex
\fun{ancestors} : \DP \to \mathcal{P}\DP$\\$
\fun{ancestors} = $\\
  \go{1}$\lambda\omega\:.\: \omega = \textit{root} \rightarrow \{\omega\},
  \{\omega\}\:\cup\:(\fun{ancestors}\:(\omega\:\vert\:(\FUN \times \FUN \times
  \DP)\elt 3))
```

```latex
\fun{commonancest} : \DP \to \DP \to \DP$\\$
\fun{commonancest} = $\\
  \go{1}$\lambda\omega_1\omega_2\:.\:$\=$
  \textrm{the only element of }$\\
  \>$\{ \omega^\prime \:\mid\:$\=$
  \omega^\prime\in(\fun{ancestors}\:\omega_1)\:\cap\:(\fun{ancestors}\:\omega_2),$\\
  \>\>$\fun{pointdepth}\:\omega^\prime\geq \fun{pointdepth}\:\omega^{\prime\prime}$\\
  \>\>$\forall
  \omega^{\prime\prime}\in(\fun{ancestors}\:\omega_1)\:\cap\:(\fun{ancestors}\:\omega_2)\}
```

```latex
\fun{pathup} : \DP \to \DP \to \arbno{(\DP \times \FUN)}$\\$
\fun{pathup} = $\\
  \go{1}$\lambda\omega_1\omega_2\:.\:
  $\=$\omega_1=\omega_2\rightarrow\langle\rangle,$\\
  \>$\langle(\omega_1, \omega_1\:\vert\:(\FUN \times \FUN \times \DP)\elt 2)\rangle
  \:\S\:$\\
  \>$(\fun{pathup}\:(\omega_1\:\vert\:(\FUN \times \FUN \times \DP)\elt 3)\omega_2)
```

```latex
\fun{pathdown} : \DP \to \DP \to \arbno{(\DP \times \FUN)}$\\$
\fun{pathdown} = $\\
  \go{1}$\lambda\omega_1\omega_2\:.\:
  $\=$\omega_1=\omega_2\rightarrow\langle\rangle,$\\
  \>$(\fun{pathdown}\:\omega_1(\omega_2\:\vert\:(\FUN \times \FUN \times \DP)\elt 3))
  \:\S\:$\\
  \>$\langle(\omega_2, \omega_2\:\vert\:(\FUN \times \FUN \times \DP)\elt 1)\rangle
```

```latex
\fun{travelpath} : \arbno{(\DP \times \FUN)} \to \CC \to \CC$\\$
\fun{travelpath} = $\\
  \go{1}$\lambda\arbno{\pi}\theta\:.\:
  $\=$\#\arbno{\pi}=0\rightarrow\theta,$\\
  \>$((\arbno{\pi}\elt 1)\elt 2)$\=$\langle\rangle((\arbno{\pi}\elt 1)\elt 1)$\\
  \>\>$(\lambda\arbno{\epsilon}\:.\:\fun{travelpath}\:(\arbno{\pi} \drop 1)\theta)
```

```latex
\fun{dynamicwind} : \arbno{\EXP} \to \DP \to \EC \to \CC$\\$
\fun{dynamicwind} = $\\
\go{1}$\fun{threearg}\,(\lambda$\=$\epsilon_1\epsilon_2\epsilon_3\omega\kappa\:.\:
  (\epsilon_1\:\elem\:\FUN\wedge\epsilon_2\:\elem\:\FUN\wedge\epsilon_3\:\elem\:\FUN)\rightarrow$\\
  \>$\fun{applicate}\:
  $\=$\epsilon_1\langle\rangle\omega(\lambda\arbno{\zeta}$\=$\:.\:$\\
  \>\>$\fun{applicate}\:$\=$\epsilon_2\langle\rangle
  ((\epsilon_1\:\vert\:\FUN,\epsilon_3\:\vert\:\FUN,\omega)\textrm{ in }\DP)$\\
  \>\>\>$(\lambda\arbno{\epsilon}\:.\:\fun{applicate}\:\epsilon_3\langle\rangle\omega(\lambda\arbno{\zeta}\:.\:\kappa\arbno{\epsilon}))),$\\
  \>$\wrong{無効手続き引数})
```

```latex
\fun{values}          :  \arbno{\EXP} \to \DP \to \EC \to \CC$\\$
\fun{values} =
 \lambda\arbno{\epsilon}\omega\kappa\:.\:\kappa\arbno{\epsilon}
```

```latex
\fun{cwv}          :  \arbno{\EXP} \to \DP \to \EC \to \CC
    \hbox{\qquad [\ide{call-with-values}]}$\\$
\fun{cwv} =$\\
 \go{1}$\fun{twoarg}\,(\lambda\epsilon_1\epsilon_2\omega\kappa\:.\:
   $\=$\fun{applicate}\:\epsilon_1\langle\:\rangle\omega
(\lambda\arbno{\epsilon}\:.\:\fun{applicate}\:\epsilon_2\:\arbno{\epsilon}\omega))
```
