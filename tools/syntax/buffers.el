;;; buffers.el --- core/tests/buffers.txt を作る (emacs -Q --batch -l buffers.el) -*- lexical-binding: t -*-
;; list-buffers の *Buffer List* の本文. 見出しは本文の 1 行目に出し, モード名は Fundamental にそろえる.
;; *Messages* と *Native-compile-Log* (rs-editor に無い) の行は除く. 使い方: OUT=core/tests/buffers.txt emacs -Q --batch -l tools/syntax/buffers.el

(setq Buffer-menu-use-header-line nil)
(advice-add 'format-mode-line :override (lambda (_f &rest _) "Fundamental"))
(dolist (n (list "short" "a-very-long-buffer-name-over-19" "名前あいうえおかきくけこさしすせそたち" " hidden" "big"))
  (with-current-buffer (get-buffer-create n)
    (insert (make-string (if (equal n "big") 1234567 12) ?x))
    (when (equal n "big") (set-buffer-modified-p nil))))
(list-buffers-noselect)
(with-temp-file (expand-file-name (or (getenv "OUT") "buffers.txt"))
  (insert (replace-regexp-in-string
           "^ %\\* \\*\\(Messages\\|Native-compile-Log\\)\\*.*\n" ""
           (with-current-buffer "*Buffer List*" (buffer-substring-no-properties (point-min) (point-max))))))
