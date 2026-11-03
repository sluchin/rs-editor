(let ((b (find-file-noselect "a.txt")))
  (with-current-buffer b
    (princ (format "%s|%s|%s\n" (buffer-name) (file-name-nondirectory (buffer-file-name)) (if (buffer-modified-p) "modified" "clean")))
    (princ (format "%s|%d\n" (buffer-string) (point)))
    (insert "x")
    (princ (format "%s\n" (if (buffer-modified-p) "modified" "clean"))))
  (princ (format "%s\n" (if (eq b (find-file-noselect "./a.txt")) "same" "different"))))
