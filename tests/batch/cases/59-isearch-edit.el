(defun state () (princ (format "%d %s [%s]\n" (point) (if (region-active-p) 1 0) (car search-ring))))
(progn (switch-to-buffer (generate-new-buffer "b"))
  (insert "foo bar Foo baz\nfoo qux\nlast FOO\nbaz foobar")
  ;; M-e: 文字を足して, 続きから探す
  (goto-char 1)
  (execute-kbd-macro (kbd "C-s fo M-e o SPC q RET RET"))
  (state)
  ;; M-e: 合った所の頭から探し直す
  (goto-char 1)
  (execute-kbd-macro (kbd "C-s ba M-e r RET RET"))
  (state)
  ;; M-e の後の DEL は, M-e の前の状態に戻る
  (goto-char 1)
  (execute-kbd-macro (kbd "C-s bar M-e DEL DEL z RET DEL RET"))
  (state)
  ;; 2 つめに合った後の M-e
  (goto-char 1)
  (execute-kbd-macro (kbd "C-s foo C-s M-e b RET RET"))
  (state)
  ;; 空にすると, 履歴の最初を使う
  (goto-char 1)
  (execute-kbd-macro (kbd "C-s x M-e DEL RET RET"))
  (state)
  ;; M-p: 履歴を直させる
  (goto-char 1)
  (execute-kbd-macro (kbd "C-s M-p RET RET"))
  (state)
  (goto-char 1)
  (execute-kbd-macro (kbd "C-s M-p M-p RET RET"))
  (state)
  (goto-char 1)
  (execute-kbd-macro (kbd "C-s M-n DEL RET RET"))
  (state)
  ;; ミニバッファの M-p は, 検索の履歴
  (goto-char 1)
  (execute-kbd-macro (kbd "C-s q M-e M-p RET RET"))
  (state)
  ;; 後ろへ
  (goto-char (point-max))
  (execute-kbd-macro (kbd "C-r ba M-e z RET RET"))
  (state)
  ;; 正規表現
  (goto-char 1)
  (execute-kbd-macro (kbd "C-M-s f.o M-e SPC RET RET"))
  (state)
  (princ (format "%s\n%s\n" (string-join search-ring "|") (string-join regexp-search-ring "|"))))
