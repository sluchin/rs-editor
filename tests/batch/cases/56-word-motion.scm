(define (walk text)
  (with-current-buffer (generate-new-buffer "t")
    (insert text)
    (goto-char (point-min))
    (let ((fwd (list)))
      (while (< (point) (point-max))
        (let ((before (point)))
          (forward-word 1)
          (set! fwd (cons (point) fwd))
          (when (= before (point)) (goto-char (point-max)))))
      (princ (format "%S fwd=%S" text (reverse fwd))))
    (let ((bwd (list)))
      (goto-char (point-max))
      (while (> (point) (point-min))
        (let ((before (point)))
          (backward-word 1)
          (set! bwd (cons (point) bwd))
          (when (= before (point)) (goto-char (point-min)))))
      (princ (format " bwd=%S\n" (reverse bwd))))))
(for-each walk '("foo_bar baz$qux 100% x-y"
                 "don't it's e.g. a.b"
                 "日本語かな漢字ひらがなカタカナ"
                 "漢字かなカナ ＡＢＣ，ＤＥＦ"
                 "é ü naïve straße"
                 "foo\tbar\nbaz  qux"
                 "（括弧）「かぎ」"
                 "ABCdef 123abc a1_b2"))
