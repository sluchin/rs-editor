;;; dump.el --- コマンドとキーの割り当てを書き出す -*- lexical-binding: t -*-

;; 環境変数 DUMP_PREFIX に, 出力のファイルの接頭辞を渡す. 次の 3 つを書く.
;;   <接頭辞>-commands.tsv  コマンド (M-x で使える記号), ライブラリ, ファイル
;;   <接頭辞>-keys.tsv      キーマップ, キー, コマンド
;;   <接頭辞>-maps.txt      書き出したキーマップの変数
;;   <接頭辞>-features.el   load-path と, 読んだ機能 (default.el が使う)
;; global-map から辿れるキーマップ (ctl-x-map, esc-map など) は, global-map のキーとして書く.

(defvar dump-prefix (getenv "DUMP_PREFIX"))

(defun dump-file (sym)
  "コマンド SYM を定義したファイル (autoload なら, そのファイル)."
  (let ((f (symbol-function sym)))
    (or (if (autoloadp f) (cadr f) (symbol-file sym 'defun)) "")))

(with-temp-file (concat dump-prefix "-commands.tsv")
  (mapatoms (lambda (s)
              (when (commandp s)
                (let ((file (dump-file s)))
                  (insert (format "%s\t%s\t%s\n" s
                                  (cond ((subr-primitive-p (indirect-function s)) "(C)")
                                        ((string= file "") "")
                                        (t (file-name-base file)))
                                  (or (locate-library file) file))))))))

(defvar dump-skip '(menu-bar tool-bar tab-bar header-line mode-line vertical-line
                    vertical-scroll-bar horizontal-scroll-bar compose-last-chars)
  "書き出さない事象 (メニューなど).")

(defun dump-submap (def)
  "DEF がプレフィックスキーなら, そのキーマップ."
  (cond ((and (symbolp def) (fboundp def) (keymapp (symbol-function def)))
         (indirect-function def))
        ((keymapp def) def)))

(defvar dump-reachable nil "global-map から辿れるキーマップ.")

(defun dump-reach (map depth)
  (when (and (< depth 6) (not (memq map dump-reachable)))
    (push map dump-reachable)
    (map-keymap (lambda (ev def)
                  (let ((sub (and (not (memq ev dump-skip)) (dump-submap def))))
                    (when sub (dump-reach sub (1+ depth)))))
                map)))

(defun dump-walk (map prefix depth mapname out)
  "MAP の割り当てを, OUT (リストを入れたコンス) に足す."
  (when (< depth 6)
    (map-keymap
     (lambda (ev def)
       (unless (or (memq ev dump-skip) (consp ev) (null def))
         (let ((keys (vconcat prefix (vector ev)))
               (sub (dump-submap def)))
           (if sub
               (dump-walk sub keys (1+ depth) mapname out)
             (push (format "%s\t%s\t%s" mapname (key-description keys)
                           (cond ((symbolp def) def)
                                 ((functionp def) "<lambda>")
                                 (t (replace-regexp-in-string "[\t\n]" " " (format "%.40S" def)))))
                   (car out))))))
     map)))

(dump-reach global-map 0)
(let ((out (list nil)) (names nil))
  (mapatoms (lambda (s)
              (when (and (boundp s) (string-match-p "-map\\'" (symbol-name s))
                         (keymapp (symbol-value s))
                         (or (eq s 'global-map)
                             (not (memq (symbol-value s) dump-reachable))))
                (push (symbol-name s) names)
                (ignore-errors (dump-walk (symbol-value s) [] 0 (symbol-name s) out)))))
  (with-temp-file (concat dump-prefix "-keys.tsv")
    (dolist (l (car out)) (insert l "\n")))
  (with-temp-file (concat dump-prefix "-maps.txt")
    (dolist (n names) (insert n "\n"))))

(with-temp-file (concat dump-prefix "-features.el")
  (prin1 (list load-path features) (current-buffer)))

;;; dump.el ends here
