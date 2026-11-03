(copy-file "a.txt" "copy.txt")
(princ (format "%s\n" (condition-case e (copy-file "a.txt" "copy.txt") (file-already-exists (error-message-string e)))))
(copy-file "multi.txt" "copy.txt" t)
(princ (format "%s\n" (condition-case e (copy-file "none.txt" "x.txt") (file-missing "missing"))))
