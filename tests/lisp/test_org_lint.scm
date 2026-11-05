;;; test_org_lint.scm --- org-lint.scm のテスト (ERT) -*- scheme -*-

;; lisp/org-lint.scm の, バッファに頼らない部分 (行の分類, 部品, チェッカー) のテスト.
;; Emacs の org-lint の結果との比較は tests/org-lint/run.sh.

(require 'org-lint)

(define (lint-kinds . lines)
  (map org-lint--kind (org-lint--scan lines)))

(define (lint-run name . lines)
  (let ((ast (list lines (org-lint--scan lines))))
    ((org-lint--checker-fun (car (filter (lambda (c) (eq? (car c) name)) org-lint--checkers))) ast)))

(ert-deftest org-lint-test-scan ()
  (should (equal (lint-kinds "* H" "SCHEDULED: <2024-01-01 Mon>" ":PROPERTIES:" ":ID: x" ":END:" "text" "")
                 '(headline planning property-drawer node-property property-drawer paragraph blank)))
  (should (equal (lint-kinds "#+BEGIN_SRC" "- not an item" "#+END_SRC")
                 '(src-block src-block src-block)))
  ;; 見出しが間にあると, ブロックは閉じていない
  (should (equal (lint-kinds "#+BEGIN_SRC" "* heading" "#+END_SRC")
                 '(paragraph headline paragraph)))
  (should (equal (lint-kinds "#+BEGIN_QUOTE" "a" "#+END_QUOTE" "#+END_QUOTE")
                 '(quote-block paragraph quote-block paragraph)))
  (should (equal (lint-kinds "#+BEGIN: clocktable" "#+END:") '(dynamic-block dynamic-block)))
  (should (equal (lint-kinds "#+TITLE: x" "#+NAME: y" "" "- a" "| a |" ": b" "# c" "-----")
                 '(keyword affiliated blank item table fixed-width comment horizontal-rule))))

(ert-deftest org-lint-test-duplicates ()
  (should (equal (org-lint--collect-duplicates '(("a" 1) ("b" 2) ("a" 3) ("a" 4)) (lambda (k) k))
                 '((3 "a") (4 "a") (1 "a")))))

(ert-deftest org-lint-test-duration ()
  (should (org-lint--duration? "1:30"))
  (should (org-lint--duration? "1d 2h"))
  (should (org-lint--duration? "1d 1:30"))
  (should-not (org-lint--duration? "abc"))
  (should-not (org-lint--duration? "3")))

(ert-deftest org-lint-test-checkers ()
  (should (equal (lint-run 'quote-section "* QUOTE a" "* ok") '((1 "Deprecated QUOTE section"))))
  (should (equal (lint-run 'misplaced-heading "a** b" "") '((7 "Possibly misplaced heading line"))))
  (should (equal (lint-run 'invalid-keyword-syntax "#+TITLE x") '((1 "Possible missing colon in keyword \"TITLE\""))))
  (should (equal (lint-run 'invalid-id-property "* H" ":PROPERTIES:" ":ID: a::b" ":END:")
                 '((18 "IDs should not include \"::\": \"a::b\"")))))
