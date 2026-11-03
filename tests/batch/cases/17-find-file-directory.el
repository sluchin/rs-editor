(let ((b (find-file-noselect "sub")))
  (with-current-buffer b
    (princ (format "%s|%s\n" (buffer-name) (if (buffer-file-name) "file" "nofile")))))
