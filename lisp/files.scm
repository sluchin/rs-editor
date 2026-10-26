;;; files.scm --- file input and output commands -*- scheme -*-

;; Emacs の files.el に当たるもの. 起動のときに読む (src-tauri/src/scheme/loadup.rs).
;; コマンドは, 引数を省くと (キーと M-x から呼ぶと), ミニバッファで聞く (Emacs の interactive に当たる).

(define-command (find-file . args)
  "Edit file FILENAME.
Switch to a buffer visiting file FILENAME, creating one if none already exists.
Interactively, read FILENAME in the minibuffer with file name completion."
  (let ((filename (if (null? args) (read-file-name "Find file: ") (car args))))
    (switch-to-buffer (find-file-noselect filename))))

(provide 'files)

;;; files.scm ends here
