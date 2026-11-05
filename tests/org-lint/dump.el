;;; Emacs の org-lint の結果を, 行, 信頼度, チェッカー, 警告の順に, タブで区切って出す.
(require 'org)
(require 'org-lint)
(org-mode)
(dolist (r (org-lint (and (boundp 'checkers) checkers)))
  (let ((v (cadr r)))
    (princ (format "%s\t%s\t%s\t%s\n" (aref v 0) (aref v 1) (org-lint-checker-name (aref v 3)) (aref v 2)))))
