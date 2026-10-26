import { describe, it, expect } from 'vitest'
import { StringStream, type StreamParser } from '@codemirror/language'
import { fileVariables, majorMode } from '@/lib/modes'
import { elispParser, type LispState, schemeParser } from '@/lib/lispMode'

/**
 * 行を, 色ごとに分けます (色を付けない字句と, 括弧は除く. 続く同じ色の字句は, 1 つにする).
 *
 * @param parser - 字句を読む関数.
 * @param lines - 行.
 * @returns 字句の文字と種類.
 */
function tokenize(parser: StreamParser<LispState>, ...lines: string[]): [string, string][] {
  // 行ごとに, 状態を写して使う (CodeMirror と同じ)
  let state = (parser.startState as (unit: number) => LispState)(2)
  const tokens: [string, string][] = []
  for (const line of lines) {
    state = (parser.copyState as (s: LispState) => LispState)(state)
    const stream = new StringStream(line, 2, 2)
    let end = -1
    while (!stream.eol()) {
      stream.start = stream.pos
      const type = parser.token(stream, state)
      expect(stream.pos).toBeGreaterThan(stream.start)
      if (!type || type === 'paren') continue
      const last = tokens[tokens.length - 1]
      if (end === stream.start && last[1] === type) last[0] += stream.current()
      else tokens.push([stream.current(), type])
      end = stream.pos
    }
  }
  return tokens
}

/** Scheme の決まりで, 色を付けた字句を返します. */
const scheme = (...lines: string[]) => tokenize(schemeParser, ...lines)
/** Emacs Lisp の決まりで, 色を付けた字句を返します. */
const elisp = (...lines: string[]) => tokenize(elispParser, ...lines)

/** Emacs の既定の字下げ. */
const INDENT = { indentTabs: true, tabWidth: 8 }

describe('majorMode', () => {
  it('バッファの名前とファイルから, メジャーモードを決めること', () => {
    expect(majorMode('*scratch*', null, '')).toEqual({
      name: 'Lisp Interaction',
      language: 'scheme',
      ...INDENT,
    })
    expect(majorMode('*Help*', null, '')).toEqual({ name: 'Help', language: null, ...INDENT })
    expect(majorMode('a.scm', '/tmp/a.scm', '')).toEqual({
      name: 'Scheme',
      language: 'scheme',
      ...INDENT,
    })
    expect(majorMode('a.sld', '/tmp/a.sld', '').language).toBe('scheme')
    expect(majorMode('.guile', '/home/u/.guile', '').language).toBe('scheme')
    expect(majorMode('a.txt', '/tmp/a.txt', '')).toEqual({
      name: 'Fundamental',
      language: null,
      ...INDENT,
    })
    expect(majorMode('a.scm', null, '').name).toBe('Fundamental')
  })

  it('Emacs Lisp のファイルは, 字句の束縛かどうかを名前に出すこと', () => {
    expect(majorMode('.emacs.el', '/home/u/.emacs.el', '(setq a 1)')).toEqual({
      name: 'ELisp/d',
      language: 'elisp',
      ...INDENT,
    })
    const lexical = ';;; init.el --- init  -*- lexical-binding: t; -*-\n(setq a 1)'
    expect(majorMode('init.el', '/home/u/init.el', lexical).name).toBe('ELisp/l')
    expect(majorMode('.emacs', '/home/u/.emacs', '').name).toBe('ELisp/d')
    expect(majorMode('_emacs', 'C:/u/_emacs', '').name).toBe('ELisp/d')
    // .dir-locals.el は, .el より先に合う
    expect(majorMode('.dir-locals.el', '/p/.dir-locals.el', '')).toEqual({
      name: 'Lisp-Data',
      language: 'elisp',
      ...INDENT,
    })
  })

  it('1 行目のファイルの変数で, メジャーモードと字下げを変えること', () => {
    const emacs = ';; -*- mode: emacs-lisp; coding: utf-8; indent-tabs-mode: nil -*-\n'
    expect(majorMode('dot.emacs', '/home/u/dot.emacs', emacs)).toEqual({
      name: 'ELisp/d',
      language: 'elisp',
      indentTabs: false,
      tabWidth: 8,
    })
    expect(majorMode('a.txt', '/tmp/a.txt', '; -*- scheme -*-').name).toBe('Scheme')
    // Org mode (ファイルの名前と, ファイルの変数)
    expect(majorMode('a.org', '/tmp/a.org', '* a')).toEqual({
      name: 'Org',
      language: null,
      indentTabs: true,
      tabWidth: 8,
    })
    expect(majorMode('a.txt', '/tmp/a.txt', '# -*- mode: org -*-').name).toBe('Org')
    expect(majorMode('a.scm', '/tmp/a.scm', '; -*- mode: fundamental -*-').name).toBe('Fundamental')
    expect(majorMode('a', '/tmp/a', '-*- mode: lisp-data; tab-width: 4 -*-')).toEqual({
      name: 'Lisp-Data',
      language: 'elisp',
      indentTabs: true,
      tabWidth: 4,
    })
    // 知らないモードは, ファイルの名前で決める
    expect(majorMode('a.el', '/tmp/a.el', '-*- mode: python -*-').name).toBe('ELisp/d')
    // ファイルでないバッファは, 本文を見ない
    expect(majorMode('*scratch*', null, '-*- indent-tabs-mode: nil -*-').indentTabs).toBe(true)
  })
})

describe('fileVariables', () => {
  it('-*- ... -*- の変数を読むこと (#! で始まれば 2 行目)', () => {
    expect(fileVariables('(a)')).toEqual({})
    expect(fileVariables('#!/bin/sh\n;; -*- mode: scheme; lexical-binding: t -*-')).toEqual({
      mode: 'scheme',
      lexicalBinding: true,
    })
    expect(
      fileVariables('-*- lexical-binding: nil; indent-tabs-mode: t; tab-width: 2 -*-'),
    ).toEqual({ lexicalBinding: false, indentTabsMode: true, tabWidth: 2 })
    // 桁の数でなければ, 使わない
    expect(fileVariables('-*- tab-width: 0; fill-column: 70 -*-')).toEqual({})
  })
})

describe('schemeParser', () => {
  it('( の直後の特別な形と, 定義する名前に色を付けること (Emacs の scheme-mode)', () => {
    expect(scheme('(define (square x) (if (car x) x 1))')).toEqual([
      ['define', 'keyword'],
      ['square', 'definedFunction'],
      ['if', 'keyword'],
    ])
    expect(scheme('(define ((curried a) b) a)')).toEqual([
      ['define', 'keyword'],
      ['curried', 'definedFunction'],
    ])
    expect(scheme('(define-syntax swap! (syntax-rules ()))')).toEqual([
      ['define-syntax', 'keyword'],
      ['swap!', 'definedVariable'],
      ['syntax-rules', 'keyword'],
    ])
    expect(scheme('(define-module (a b))')).toEqual([
      ['define-module', 'keyword'],
      ['a', 'typeName'],
    ])
    // define-command は, Emacs と同じく色を付けない
    expect(scheme('(define-command hello (lambda () 1))')).toEqual([['lambda', 'keyword']])
    expect(scheme('(let loop ((i 0)) i) (let ((j 1)) j)')).toEqual([
      ['let', 'keyword'],
      ['loop', 'definedFunction'],
      ['let', 'keyword'],
    ])
    expect(scheme('(library (srfi 1) (import))')).toEqual([
      ['library', 'keyword'],
      ['srfi', 'typeName'],
      ['import', 'keyword'],
    ])
    // ( の直後でなければ, キーワードでも色を付けない. [ の後も付けない
    expect(scheme("(list if 'define) [if]")).toEqual([])
    // `'(define y)` は, Emacs と同じく色を付ける
    expect(scheme("'(define y)")).toEqual([
      ['define', 'keyword'],
      ['y', 'definedFunction'],
    ])
    // 定義する名前は, 次の字句で, 同じ行にあるものだけ
    expect(scheme('(define "x" y)', '(define', ' z)')).toEqual([
      ['define', 'keyword'],
      ['"x"', 'string'],
      ['define', 'keyword'],
    ])
  })

  it('型と, : で始まる名前に色を付けること', () => {
    expect(scheme('(make <point> :x #:y : x)')).toEqual([
      ['<point>', 'typeName'],
      [':x', 'builtin'],
      ['#:y', 'builtin'],
    ])
  })

  it('コメントと文字列は, 行をまたいで色を付けること', () => {
    expect(scheme('(+ 1 2) ; sum (if')).toEqual([['; sum (if', 'comment']])
    expect(scheme('"a \\" (if', 'b" (when)')).toEqual([
      ['"a \\" (if', 'string'],
      ['b"', 'string'],
      ['when', 'keyword'],
    ])
    expect(scheme('#| a #| b |# (if', 'c |# (or)')).toEqual([
      ['#| a #| b |# (if', 'comment'],
      ['c |#', 'comment'],
      ['or', 'keyword'],
    ])
  })

  it('#; の後の式 1 つを, コメントにすること', () => {
    const comments = (line: string) =>
      scheme(line)
        .filter(([, type]) => type === 'comment')
        .map(([text]) => text)
        .join('')
    expect(comments('#;(if (a) "b)") (when)')).toBe('#;(if (a) "b)")')
    expect(scheme('#;(if', 'x) (when)').pop()).toEqual(['when', 'keyword'])
    expect(comments("#; 'x (if)")).toBe("#; 'x")
    expect(comments('#;"abc')).toBe('#;"abc')
    expect(comments('#;[a] (if)')).toBe('#;[a]')
  })

  it('文字の括弧と, 名前でない文字を, 名前と分けること', () => {
    // `#\(` は文字なので, 次の `(if` は ( の直後
    expect(scheme('(list #\\( `(if ,x))')).toEqual([['if', 'keyword']])
    expect(scheme('#\\')).toEqual([])
  })
})

describe('elispParser', () => {
  it('定義の形と, 定義する名前に色を付けること (Emacs の emacs-lisp-mode)', () => {
    expect(elisp('(defun my-fn (a &optional b) (car a))')).toEqual([
      ['defun', 'keyword'],
      ['my-fn', 'definedFunction'],
      ['&optional', 'typeName'],
    ])
    expect(elisp("(defvar v 1) (defalias 'f #'g) (defgroup my nil)")).toEqual([
      ['defvar', 'keyword'],
      ['v', 'definedVariable'],
      ['defalias', 'keyword'],
      ['f', 'definedFunction'],
      ['defgroup', 'keyword'],
      ['my', 'typeName'],
    ])
    // `(` の後の名前は, 関数なら色を付けず, 型なら付ける
    expect(elisp('(defun (f)) (cl-defstruct (st (:copier nil)))')).toEqual([
      ['defun', 'keyword'],
      ['cl-defstruct', 'keyword'],
      ['st', 'typeName'],
      [':copier', 'builtin'],
    ])
  })

  it('マクロ, 機能の名前, 警告の形に色を付けること', () => {
    expect(elisp('(require \'cl-lib) (throw (x)) (error "x") (when t) (car t)')).toEqual([
      ['require', 'keyword'],
      ['cl-lib', 'constant'],
      ['throw', 'keyword'],
      ['error', 'warning'],
      ['"x"', 'string'],
      ['when', 'keyword'],
    ])
  })

  it("'( と let の束縛の中では, マクロに色を付けないこと", () => {
    expect(elisp("(defvar x '(progn)) '(a (when b))")).toEqual([
      ['defvar', 'keyword'],
      ['x', 'definedVariable'],
      ['when', 'keyword'],
    ])
    expect(elisp('(let ((when 1)) (cond ((progn 1))))')).toEqual([
      ['let', 'keyword'],
      ['cond', 'keyword'],
      ['progn', 'keyword'],
    ])
    // `` `( `` と `#'(` は, 関数を呼ぶリスト
    expect(elisp("`(when ,(progn 1)) #'(lambda () 1)")).toEqual([
      ['when', 'keyword'],
      ['progn', 'keyword'],
      ['lambda', 'keyword'],
    ])
  })

  it('文字列の中の, 正規表現のグループ, キー, 名前, 要らない \\ に色を付けること', () => {
    expect(elisp('"\\\\(a\\\\|b\\\\)\\\\(?:c\\\\) \\\\\\\\(d"')).toEqual([
      ['"', 'string'],
      ['\\\\', 'regexpBackslash'],
      ['(', 'regexpConstruct'],
      ['a', 'string'],
      ['\\\\', 'regexpBackslash'],
      ['|', 'regexpConstruct'],
      ['b', 'string'],
      ['\\\\', 'regexpBackslash'],
      [')', 'regexpConstruct'],
      ['\\\\', 'regexpBackslash'],
      ['(?:', 'regexpConstruct'],
      ['c', 'string'],
      ['\\\\', 'regexpBackslash'],
      [')', 'regexpConstruct'],
      // `\\\\` は, 正規表現の `\` そのもの
      [' \\\\\\\\(d"', 'string'],
    ])
    expect(elisp('"\\\\[cmd] \\\\<map> \\\\= `name\' \\. \\n"')).toEqual([
      ['"\\\\[', 'string'],
      ['cmd', 'constant'],
      ['] \\\\<', 'string'],
      ['map', 'definedVariable'],
      ['> ', 'string'],
      ['\\\\=', 'builtin'],
      [' `', 'string'],
      ['name', 'constant'],
      ["' ", 'string'],
      ['\\', 'warning'],
      ['. \\n"', 'string'],
    ])
    // 行の始めの `\(` は, 要るもの
    expect(elisp('"a', '\\(b"')).toEqual([
      ['"a', 'string'],
      ['\\(b"', 'string'],
    ])
  })

  it("コメントの中の `name' に色を付けること", () => {
    expect(elisp(";; see `foo' and `bar'", '(when t)')).toEqual([
      [';; see `', 'comment'],
      ['foo', 'constant'],
      ["' and `", 'comment'],
      ['bar', 'constant'],
      ["'", 'comment'],
      ['when', 'keyword'],
    ])
  })

  it('前の行で開いた括弧を閉じた後ろに続く引数を, 行の終わりまで警告すること', () => {
    expect(elisp('(foo (a', '      b) c "s")')).toEqual([['c "s")', 'warning']])
    expect(elisp('(foo (a', '      b)) c')).toEqual([['c', 'warning']])
    // 閉じる括弧と, コメントと, 行の終わりは, 警告しない
    expect(elisp('(foo (a', '      b) ; x', '  b))')).toEqual([['; x', 'comment']])
    // 同じ行で開いた括弧は, 警告しない
    expect(elisp('(foo (a b) c)')).toEqual([])
  })

  it('文字の括弧を, 名前の文字にすること', () => {
    expect(elisp('(list ?( ?\\( ?a) (when)')).toEqual([['when', 'keyword']])
  })
})
