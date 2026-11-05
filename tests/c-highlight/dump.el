;;; Emacs の c-mode の font-lock の結果を, 範囲ごとに, 始まり, 終わり, 種類の順に, タブで区切って出す.
;;; 位置は 1 から数え, 終わりは含まない. 種類は rs-editor の `token-spans' と同じ名前. 字句だけで決まる種類に絞る
;;; (宣言の名前の色と, 組み込みでない型と定数の名前は出さない). 範囲の前後の改行は, 含めない (dump.scm も同じ).
(setq enable-local-variables nil)
(require 'cc-mode)
(defun dump--word-at (pos)
  "POS を含む語."
  (save-excursion
    (goto-char pos)
    (skip-chars-backward "[:alnum:]_$")
    (let ((start (point)))
      (skip-chars-forward "[:alnum:]_$")
      (buffer-substring-no-properties start (point)))))
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
      ('font-lock-type-face
       (and (eq 0 (string-match c-primitive-type-key (dump--word-at pos))) 'type))
      ('font-lock-constant-face
       (and (eq 0 (string-match c-constant-key (dump--word-at pos))) 'constant))
      (_ nil))))
(let ((runs nil))
  (c-mode)
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
