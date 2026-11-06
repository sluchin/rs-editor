;; M-q: Lisp の文字列 (docstring) の中を詰める. リージョンがあれば, リージョンの段落を全て詰める
(transient-mark-mode 1)
(defun run (label text pos &optional region-end)
  (with-current-buffer (generate-new-buffer "t")
    (scheme-mode)
    (setq fill-column 40)
    (insert text)
    (goto-char pos)
    (when region-end (set-mark region-end) (activate-mark))
    (call-interactively 'fill-paragraph)
    (princ (format "== %s: point=%d\n%S\n" label (point) (buffer-string)))))
(let ((doc "(define (foo)\n  \"This is a very long documentation string that should be filled into several lines when fill paragraph runs inside it, okay.\"\n  #t)\n"))
  (run "doc-inside" doc 40)
  (run "doc-two-paragraphs"
       "(define (foo)\n  \"First paragraph of the documentation that is long enough to be filled.\n\nSecond paragraph of the documentation that is also long enough to be filled.\"\n  #t)\n"
       30)
  (run "code" "(define (foo) (bar baz) (qux quux corge grault garply waldo fred plugh xyzzy thud))\n" 5)
  (run "comment" ";; this is a long comment that should be filled into several lines when the command runs\n(foo)\n" 10))
(run "region"
     "aaa bbb ccc ddd eee fff ggg hhh iii jjj kkk lll mmm nnn ooo ppp qqq\n\nsecond paragraph is long enough to be filled too when the region covers it fully\n\nthird that is outside of the region and is also long enough to be filled somewhere\n"
     1 160)
