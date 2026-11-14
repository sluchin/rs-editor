#!/bin/sh
# Emacs のバッチモードと rs-editor のバッチモードで同じ試験を動かし, 結果を比べる.
#
# 使い方: tests/batch/run.sh [-e] [-k] [-v] [-l] [試験の名前の一部...]
#   -e  標準エラー出力 (message など. 標準出力とは別に比べる) も比べる (既定は標準出力と終了コードと作ったファイル)
#   -k  作業用のディレクトリを消さずに残す
#   -v  失敗したときだけでなく, 全ての出力を出す
#   -l  試験の一覧を出して終わる
# 環境変数:
#   EMACS      Emacs の実行ファイル (既定は emacs)
#   RSEDITOR   rs-editor の実行ファイル (既定は target/debug/rseditor-tui. 無ければ cargo build する)
#
# 標準入力は cases/NAME.in (あれば). 無ければ空.
# cases/NAME.sh (あれば) は, 動かす前に, 作業用のディレクトリで sh で動かす (実行できるファイルやシンボリックリンクを作るなど).
# 試験は cases/NAME.el (Emacs Lisp) と cases/NAME.scm (Scheme) の対. どちらも空のディレクトリで動かし,
#   標準出力, 終了コード, 動かした後のディレクトリのファイル (名前と中身) を比べる.
# 動かす前に, 作業用のディレクトリに次のファイルを置く.
#   a.txt (old), multi.txt (line1 line2 line3), sub/ (空のディレクトリ)
# 作業用のディレクトリの名前は, 比べる前に @T@ に置き換える. Emacs 30 が標準出力に出す「Error: ...」の行は, 比べない.
set -u
# メッセージの言語をそろえる (Emacs はロケールでエラーの文を訳す)
LC_ALL=C
export LC_ALL

here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../.." && pwd)
EMACS=${EMACS:-emacs}
RSEDITOR=${RSEDITOR:-$root/target/debug/rseditor-tui}
stderr=0 keep=0 verbose=0 list=0
while getopts ekvl opt; do
  case $opt in
    e) stderr=1 ;; k) keep=1 ;; v) verbose=1 ;; l) list=1 ;;
    *) sed -n '2,/^set -u/p' "$0" | sed '$d;s/^# \{0,1\}//'; exit 2 ;;
  esac
done
shift $((OPTIND - 1))

# 試験の一覧 (cases/*.el と, 下の CLI の試験)
cli_names="cli-eval cli-load-order cli-funcall cli-visit-file cli-script"
names=$(for f in "$here"/cases/*.el; do basename "$f" .el; done; for n in $cli_names; do echo "$n"; done)
if [ "$list" = 1 ]; then echo "$names"; exit 0; fi
if [ $# -gt 0 ]; then
  selected=
  for n in $names; do
    for pat in "$@"; do case $n in *"$pat"*) selected="$selected $n"; break ;; esac; done
  done
  names=$selected
fi

command -v "$EMACS" >/dev/null || { echo "emacs が見つかりません ($EMACS)" >&2; exit 2; }
# 既定の実行ファイルは, いつも作り直す (カバレッジの計測が, 計測つきのものに置き換えることがある)
if [ "$RSEDITOR" = "$root/target/debug/rseditor-tui" ]; then
  (cd "$root" && cargo build -q -p rseditor-tui) || exit 2
fi
[ -x "$RSEDITOR" ] || { echo "rseditor-tui が見つかりません ($RSEDITOR)" >&2; exit 2; }
# 計測つきの実行ファイルが, 作業用のディレクトリにファイルを書かないようにする
LLVM_PROFILE_FILE=/dev/null
export LLVM_PROFILE_FILE
# Emacs は /tmp の下のファイルをバックアップしないので, リポジトリの target の下で動かす
mkdir -p "$root/target"
tmp=$(mktemp -d "$root/target/batch-test.XXXXXX")
[ "$keep" = 1 ] && echo "作業用のディレクトリ: $tmp" >&2
cleanup() { [ "$keep" = 1 ] || rm -rf "$tmp"; }
trap cleanup EXIT

# 作業用のディレクトリを作り, 置くファイル
fixture() {
  mkdir -p "$1/sub"
  printf 'old\n' > "$1/a.txt"
  printf 'line1\nline2\nline3\n' > "$1/multi.txt"
}

# run DIR LABEL コマンド... : DIR で動かし, DIR.out に出力を書く
run() {
  dir=$1; shift
  fixture "$dir"
  [ -n "${setup:-}" ] && (cd "$dir" && sh "$setup")
  (
    cd "$dir" || exit 2
    # 標準出力と標準エラー出力は, 別々に書く (Emacs は標準出力を溜めてから出すので, 混ぜると順序が合わない)
    "$@" < "${input:-/dev/null}" 2> "$dir.err"
    echo "== status: $?"
    if [ "$stderr" = 1 ]; then
      echo "== stderr"
      cat "$dir.err"
    fi
    echo "== files"
    find . -type f | LC_ALL=C sort | while read -r f; do
      echo "--- $f"
      cat "$f"
    done
  ) | sed "s|$dir|@T@|g" | sed -e ":a;N;\$!ba;s/\\n\\nError: .*\\n== status:/\\n== status:/;s/\\nError: .*\\n== status:/== status:/" > "$dir.out"
}

emacs_cmd() { timeout 60 "$EMACS" -Q --batch --eval '(setq create-lockfiles nil)' "$@"; }
rs_cmd() { timeout 60 "$RSEDITOR" --batch "$@"; }

pass=0 fail=0 skip=0
for name in $names; do
  case_dir=$tmp/$name
  mkdir -p "$case_dir"
  e=$case_dir/emacs r=$case_dir/rs
  case $name in
    cli-eval)
      run "$e" emacs_cmd --eval '(princ (format "%d\n" (+ 1 2)))' --eval '(princ "two\n")'
      run "$r" rs_cmd --eval '(princ (format "%d\n" (+ 1 2)))' --eval '(princ "two\n")' ;;
    cli-load-order)
      printf '(princ "first\\n")\n' > "$case_dir/1.el"; printf '(princ "second\\n")\n' > "$case_dir/2.el"
      printf '(princ "first\\n")\n' > "$case_dir/1.scm"; printf '(princ "second\\n")\n' > "$case_dir/2.scm"
      run "$e" emacs_cmd -l "$case_dir/1.el" --eval '(princ "middle\n")' -l "$case_dir/2.el"
      run "$r" rs_cmd -l "$case_dir/1.scm" --eval '(princ "middle\n")' -l "$case_dir/2.scm" ;;
    cli-funcall)
      printf '(defun hello () (princ "hello\\n"))\n' > "$case_dir/f.el"
      printf '(define (hello) (princ "hello\\n"))\n' > "$case_dir/f.scm"
      run "$e" emacs_cmd -l "$case_dir/f.el" -f hello
      run "$r" rs_cmd -l "$case_dir/f.scm" -f hello ;;
    cli-visit-file)
      run "$e" emacs_cmd a.txt --eval '(princ (format "%s|%s" (buffer-name) (buffer-string)))'
      run "$r" rs_cmd a.txt --eval '(princ (format "%s|%s" (buffer-name) (buffer-string)))' ;;
    cli-script)
      printf '(princ "script\\n")\n' > "$case_dir/s.el"; printf '(princ "script\\n")\n' > "$case_dir/s.scm"
      run "$e" emacs_cmd --script "$case_dir/s.el"
      run "$r" rs_cmd --script "$case_dir/s.scm" ;;
    *)
      # 標準入力は, cases/NAME.in があればそれ (無ければ空)
      input=/dev/null
      [ -f "$here/cases/$name.in" ] && input="$here/cases/$name.in"
      setup=
      [ -f "$here/cases/$name.sh" ] && setup="$here/cases/$name.sh"
      run "$e" emacs_cmd -l "$here/cases/$name.el"
      if [ -f "$here/cases/$name.scm" ]; then
        run "$r" rs_cmd -l "$here/cases/$name.scm"
      else
        echo "SKIP  $name (cases/$name.scm が無い)"; skip=$((skip + 1)); continue
      fi ;;
  esac
  input=/dev/null setup=
  if diff -u "$e.out" "$r.out" > "$case_dir/diff" 2>&1; then
    echo "PASS  $name"; pass=$((pass + 1))
    [ "$verbose" = 1 ] && sed 's/^/    /' "$e.out"
  else
    echo "FAIL  $name"; fail=$((fail + 1))
    sed -e '1,2d' -e 's/^/    /' "$case_dir/diff"
  fi
done
echo "pass $pass, fail $fail, skip $skip"
[ "$fail" = 0 ]
