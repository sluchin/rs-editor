(with-current-buffer (find-file-noselect "new.txt")
  (princ (format "%s|%d|%s|%s\n" (buffer-name) (buffer-size) (file-name-nondirectory (buffer-file-name)) (if (buffer-modified-p) "modified" "clean"))))
