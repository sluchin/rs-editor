#!/usr/bin/env python3
"""組み込みの手続き (Rust) のうち, Emacs に同じ名前があるものの docstring を, Emacs から書き出す.

使い方: tools/emacs-commands/builtin_docs.py
  core/src/scheme/*.rs の `register!` に書いた名前を集め, `emacs -Q --batch` で docstring を引き,
  core/src/scheme/emacs_docs.rs に書く (名前の順). Emacs に無い名前は, 手で書く core/src/scheme/builtin_docs.rs に置く.
"""
import json
import os
import re
import subprocess
import sys
import tempfile

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..'))
SCHEME = os.path.join(ROOT, 'core', 'src', 'scheme')

ELISP = r'''
(require 'json)
(let ((names (with-temp-buffer
               (insert-file-contents (getenv "NAMES_FILE"))
               (split-string (buffer-string) "\n" t)))
      (out nil))
  (dolist (name names)
    (let* ((s (intern-soft name))
           (doc (and s (fboundp s) (documentation s t))))
      (when doc
        (push (cons name (replace-regexp-in-string "\n\n(fn\\( .*\\)?)\\'" "" doc)) out))))
  (with-temp-file (getenv "OUT_FILE")
    (insert (json-encode (nreverse out)))))
'''


def register_names():
    """`register!` の中の名前."""
    names = []
    for file in sorted(os.listdir(SCHEME)):
        if not file.endswith('.rs'):
            continue
        text = open(os.path.join(SCHEME, file), encoding='utf-8').read()
        for block in re.finditer(r'register!\(\s*\w+,(.*?)\n\s*\);', text, re.S):
            names += re.findall(r'"((?:[^"\\]|\\.)+)"\s*=>', block.group(1))
        # editor_ops.rs は, SPECS の表の名前を全て登録する
        specs = re.search(r'const SPECS.*?\n\];', text, re.S)
        if specs:
            names += re.findall(r'^\s+\(\s*"([^"]+)",', specs.group(0), re.M)
    return sorted(set(names))


def rust_string(text):
    """Rust の文字列リテラル."""
    out = ['"']
    for ch in text:
        if ch == '"':
            out.append('\\"')
        elif ch == '\\':
            out.append('\\\\')
        elif ch == '\n':
            out.append('\\n')
        elif ch == '\t':
            out.append('\\t')
        elif ord(ch) < 0x20:
            out.append('\\u{%x}' % ord(ch))
        else:
            out.append(ch)
    out.append('"')
    return ''.join(out)


def main():
    names = register_names()
    with tempfile.TemporaryDirectory() as tmp:
        names_file = os.path.join(tmp, 'names.txt')
        out_file = os.path.join(tmp, 'docs.json')
        script = os.path.join(tmp, 'docs.el')
        open(names_file, 'w', encoding='utf-8').write('\n'.join(names))
        open(script, 'w', encoding='utf-8').write(ELISP)
        env = dict(os.environ, NAMES_FILE=names_file, OUT_FILE=out_file)
        subprocess.run(['emacs', '-Q', '--batch', '-l', script], env=env, check=True,
                       stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        docs = dict(json.load(open(out_file, encoding='utf-8')))
    lines = [
        '//! Emacs の docstring (組み込みの手続きのうち, Emacs に同じ名前があるもの).',
        '//!',
        '//! `tools/emacs-commands/builtin_docs.py` が `emacs -Q --batch` から作ります. 手で直さないでください.',
        '//! 動きが Emacs と違うものと, Emacs に無いものは, `builtin_docs.rs` に書きます (そちらが先).',
        '',
        '/// 名前と docstring (名前の順).',
        'pub static DOCS: &[(&str, &str)] = &[',
    ]
    for name in names:
        if name in docs:
            lines.append('    (%s, %s),' % (rust_string(name), rust_string(docs[name])))
    lines.append('];')
    path = os.path.join(SCHEME, 'emacs_docs.rs')
    open(path, 'w', encoding='utf-8').write('\n'.join(lines) + '\n')
    missing = [n for n in names if n not in docs]
    print('%d names, %d from Emacs, %d to write by hand' % (len(names), len(docs), len(missing)))
    open(os.path.join(os.environ.get('TMPDIR', '/tmp'), 'builtin_docs_missing.txt'), 'w').write('\n'.join(missing))
    return 0


if __name__ == '__main__':
    sys.exit(main())
