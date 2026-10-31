;;; bookmarks.el --- core/tests/bookmarks.txt を作る (emacs -Q --batch -l bookmarks.el) -*- lexical-binding: t -*-
;; bookmark-bmenu-list の *Bookmark List* の本文. 使い方: OUT=core/tests/bookmarks.txt emacs -Q --batch -l tools/syntax/bookmarks.el

(require 'bookmark)
(setq bookmark-alist
      (list '("foo" (filename . "/tmp/foo.txt") (position . 1))
            '("Bar" (filename . "~/x/y.txt") (annotation . "hi"))
            '("a-name-that-is-longer-than-thirty-nine-characters-xx" (filename . "/z"))
            '("名前あいう" (filename . "/m"))
            '("baz" (filename . "/b"))
            '("nofile" (position . 3))))
(setq bookmark-alist-modification-count 0)
(bookmark-bmenu-list)
(with-temp-file (expand-file-name (or (getenv "OUT") "bookmarks.txt"))
  (insert (with-current-buffer "*Bookmark List*" (buffer-substring-no-properties (point-min) (point-max)))))
