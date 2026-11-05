import { describe, it, expect } from 'vitest'
import { fileVariables, majorMode } from '@/lib/modes'

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
    expect(majorMode('*grep*', null, '')).toMatchObject({ name: 'Grep', language: null })
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
    // C (ファイルの名前と、ファイルの変数. 色は核が付けるので、言語は無い)
    expect(majorMode('a.c', '/tmp/a.c', 'int x;')).toEqual({
      name: 'C',
      language: null,
      indentTabs: true,
      tabWidth: 8,
    })
    expect(majorMode('a.h', '/tmp/a.h', '').name).toBe('C')
    // C++
    expect(majorMode('a.cpp', '/tmp/a.cpp', 'int x;')).toEqual({
      name: 'C++',
      language: null,
      indentTabs: true,
      tabWidth: 8,
    })
    expect(majorMode('a.hpp', '/tmp/a.hpp', '').name).toBe('C++')
    expect(majorMode('a.c++', '/tmp/a.c++', '').name).toBe('C++')
    expect(majorMode('a.txt', '/tmp/a.txt', '// -*- mode: c++ -*-').name).toBe('C++')
    expect(majorMode('a.txt', '/tmp/a.txt', '/* -*- mode: c -*- */').name).toBe('C')
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
