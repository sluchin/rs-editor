;;; Emacs の c-mode の font-lock の結果を, 範囲ごとに, 始まり, 終わり, 種類の順に, タブで区切って出す.
;;; 位置は 1 から数え, 終わりは含まない. 種類は rs-editor の `token-spans' と同じ名前 (宣言の名前も含む).
;;; 範囲の前後の改行は, 含めない (dump.scm も同じ).
;;; ファイルの名前に "lexical" が付くものは, 字句だけで決まる種類に絞る (型, 定数, 関数と変数の名前は出さない).
;;; C++ の, 宣言の名前を扱わない構文 (テンプレート, 名前空間, クラスなど) を含むケースに使う.
(setq enable-local-variables nil)
(require 'cc-mode)
(defconst dump--lexical (string-match-p "lexical" (buffer-file-name)))
(defun dump--kind (pos)
  "POS の文字の種類 (無ければ nil)."
  (let ((face (get-text-property pos 'face)))
    (when (consp face) (setq face (if (memq 'font-lock-doc-face face) 'font-lock-doc-face (car face))))
    (pcase face
      ('font-lock-comment-face 'comment)
      ('font-lock-comment-delimiter-face 'comment-delimiter)
      ('font-lock-doc-face 'doc)
      ('font-lock-string-face 'string)
      ('font-lock-keyword-face 'keyword)
      ('font-lock-preprocessor-face 'preprocessor)
      ((and 'font-lock-type-face (guard (not dump--lexical))) 'type)
      ((and 'font-lock-constant-face (guard (not dump--lexical))) 'constant)
      ((and 'font-lock-function-name-face (guard (not dump--lexical))) 'function-name)
      ((and 'font-lock-variable-name-face (guard (not dump--lexical))) 'variable-name)
      (_ nil))))
(let ((runs nil))
  (if (string-match-p "\\.cpp\\'" (buffer-file-name)) (c++-mode) (c-mode))
  (font-lock-mode 1)
  (font-lock-ensure)
  (let ((pos (point-min)))
    (while (< pos (point-max))
      (let ((kind (dump--kind pos)))
        (when kind
          (if (and runs (eq (nth 2 (car runs)) kind) (= (nth 1 (car runs)) pos))
              (setcar (cdr (car runs)) (1+ pos))
            (push (list pos (1+ pos) kind) runs))))
      (setq pos (1+ pos))))
  (dolist (run (nreverse runs))
    (let ((start (nth 0 run)) (end (nth 1 run)))
      (while (and (< start end) (eq (char-after start) ?\n)) (setq start (1+ start)))
      (while (and (< start end) (eq (char-before end) ?\n)) (setq end (1- end)))
      (when (< start end)
        (princ (format "%d\t%d\t%s\n" start end (nth 2 run)))))))
