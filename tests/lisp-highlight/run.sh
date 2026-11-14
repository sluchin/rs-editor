#!/bin/sh
# Emacs の scheme-mode と emacs-lisp-mode の font-lock と, rs-editor の Lisp のシンタックスハイライト (core/src/highlight/lisp.rs) を,
# 同じファイルで動かし, 結果を比べる.
#
# 使い方: tests/lisp-highlight/run.sh [-v] [試験の名前の一部...]
#   -v  成功したものも, 結果を出す
# 環境変数:
#   EMACS      Emacs の実行ファイル (既定は emacs)
#   RSEDITOR   rs-editor の実行ファイル (既定は target/debug/rseditor-tui. 無ければ cargo build する)
#
# 試験は cases/NAME.scm (Scheme) と cases/NAME.el (Emacs Lisp). 比べるのは, 範囲 (始まりと終わり) と種類
# (dump.el と, tests/c-highlight/dump.scm が, 同じ形で出す).
set -u
LC_ALL=C
export LC_ALL
here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../.." && pwd)
EMACS=${EMACS:-emacs}
RSEDITOR=${RSEDITOR:-$root/target/debug/rseditor-tui}
verbose=0
while getopts v opt; do
  case $opt in v) verbose=1 ;; *) sed -n '2,/^set -u/p' "$0" | sed '$d;s/^# \{0,1\}//'; exit 2 ;; esac
done
shift $((OPTIND - 1))
command -v "$EMACS" >/dev/null || { echo "emacs が見つかりません ($EMACS)" >&2; exit 2; }
if [ "$RSEDITOR" = "$root/target/debug/rseditor-tui" ]; then
  (cd "$root" && cargo build -q -p rseditor-tui) || exit 2
fi
LLVM_PROFILE_FILE=/dev/null
export LLVM_PROFILE_FILE
tmp=$(mktemp -d "$root/target/lisp-highlight-test.XXXXXX")
trap 'rm -rf "$tmp"' EXIT
pass=0 fail=0
for f in "$here"/cases/*.scm "$here"/cases/*.el; do
  file=$(basename "$f"); name=${file%.*}
  if [ $# -gt 0 ]; then
    ok=0; for pat in "$@"; do case $name in *"$pat"*) ok=1 ;; esac; done
    [ $ok = 1 ] || continue
  fi
  (cd "$here/cases" && "$EMACS" -Q --batch "$file" -l "$here/dump.el" >"$tmp/$name.emacs" 2>"$tmp/$name.emacs.err")
  (cd "$here/cases" && "$RSEDITOR" --batch "$file" -l "$root/tests/c-highlight/dump.scm" >"$tmp/$name.rs" 2>"$tmp/$name.rs.err")
  if [ -s "$tmp/$name.emacs.err" ] || [ -s "$tmp/$name.rs.err" ]; then
    fail=$((fail + 1)); echo "FAIL  $name (標準エラー出力あり)"
    sed 's/^/    emacs: /' "$tmp/$name.emacs.err" | head -5
    sed 's/^/    rs: /' "$tmp/$name.rs.err" | head -5
  elif cmp -s "$tmp/$name.emacs" "$tmp/$name.rs"; then
    pass=$((pass + 1)); echo "PASS  $name"
    [ $verbose = 1 ] && sed 's/^/    /' "$tmp/$name.rs"
  else
    fail=$((fail + 1)); echo "FAIL  $name"
    diff "$tmp/$name.emacs" "$tmp/$name.rs" | sed 's/^/    /'
  fi
done
echo "pass $pass, fail $fail"
[ $fail = 0 ]
