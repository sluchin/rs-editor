;;; rs-editor の org-lint の結果を, dump.el と同じ形で出す.
(for-each
 (lambda (r)
   (let ((v (cadr r)))
     (princ (format "%s\t%s\t%s\t%s\n" (list-ref v 0) (list-ref v 1) (list-ref v 3) (list-ref v 2)))))
 (org-lint-reports))
