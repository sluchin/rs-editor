#!/usr/bin/env python3
"""cases/NAME.el (Emacs Lisp) から cases/NAME.scm (Scheme) を作る. 書き方の決まり:
  - nil は #f, t は #t にする (文字列の中の ` nil ` ` t ` は避ける)
  - (setq X V) は (set! X V) にする. defun, dolist は使わない
使い方: tools/gen.py [NAME...]   (既定は, .scm の無い .el 全部. -f で全部作り直す)
"""
import re
import sys
from pathlib import Path

cases = Path(__file__).resolve().parent.parent / 'cases'
force = '-f' in sys.argv
names = [a for a in sys.argv[1:] if a != '-f']


def convert(text):
    out = []
    for part in re.split(r'("(?:[^"\\]|\\.)*")', text):
        if part.startswith('"'):
            out.append(part)
        else:
            part = re.sub(r"(?<![\w\-#'])nil(?![\w\-])", '#f', part)
            part = re.sub(r'(?<![\w\-#\'])t(?![\w\-])', '#t', part)
            part = part.replace('(setq ', '(set! ').replace('(progn ', '(begin ')
            part = re.sub(r'\(defun ([\w\-]+) \(', r'(define (\1 ', part)
            part = re.sub(r'\(defvar ([\w\-]+) ', r'(define \1 ', part)
            part = re.sub(r'^\(require [^\n]*\n', '', part)
            # 文字の記法 ?a は, 1 文字の文字列にする
            part = re.sub(r'(?<![\w\-<>=/!*+])\?(\\.|[^\s()\\])', lambda m: '"' + m.group(1) + '"', part)
            out.append(part)
    return ''.join(out)


for el in sorted(cases.glob('*.el')):
    name = el.stem
    # 01-17 の .scm は、手で書いたもの (作り直さない)
    if int(name.split('-')[0]) < 20 and not names:
        continue
    if names and name not in names:
        continue
    scm = el.with_suffix('.scm')
    # 手で直した .scm (先頭が ";;; hand-written") は、作り直さない
    if scm.exists() and scm.read_text().startswith(';;; hand-written'):
        continue
    if scm.exists() and not force and not names:
        continue
    scm.write_text(convert(el.read_text()))
    print('generated', scm.name)
