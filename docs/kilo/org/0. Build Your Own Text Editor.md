  Build Your Own Text Editor 

[](https://viewsourcecode.org/snaptoken/kilo/#)[open the booklet](https://viewsourcecode.org/snaptoken/kilo/01.setup.html)[](https://viewsourcecode.org/snaptoken/kilo/#)

Build Your Own Text Editor
==========================

Welcome! This is an instruction booklet that shows you how to build a text editor in C.

The text editor is [antirez窶冱 kilo](http://antirez.com/news/108), with some changes. It窶冱 about 1000 lines of C in a single file with no dependencies, and it implements all the basic features you expect in a minimal editor, as well as syntax highlighting and a search feature.

This booklet walks you through building the editor in **184 steps**. Each step, you窶冤l add, change, or remove a few lines of code. Most steps, you窶冤l be able to **observe the changes** you made by compiling and running the program immediately afterwards.

I explain each step along the way, sometimes in a lot of detail. Feel free to skim or skip the prose, as the main point of this is that **you are going to build a text editor from scratch**! Anything you learn along the way is bonus, and there窶冱 plenty to learn just from typing in the changes to the code and observing the results.

See the [appendices](https://viewsourcecode.org/snaptoken/kilo/08.appendices.html) for more information on the tutorial itself (including what to do if you get stuck, and where to get help).

If you窶决e ready to begin, then go to [chapter 1](https://viewsourcecode.org/snaptoken/kilo/01.setup.html)!

Table of Contents
-----------------

1.  [Setup](https://viewsourcecode.org/snaptoken/kilo/01.setup.html)
2.  [Entering raw mode](https://viewsourcecode.org/snaptoken/kilo/02.enteringRawMode.html)
3.  [Raw input and output](https://viewsourcecode.org/snaptoken/kilo/03.rawInputAndOutput.html)
4.  [A text viewer](https://viewsourcecode.org/snaptoken/kilo/04.aTextViewer.html)
5.  [A text editor](https://viewsourcecode.org/snaptoken/kilo/05.aTextEditor.html)
6.  [Search](https://viewsourcecode.org/snaptoken/kilo/06.search.html)
7.  [Syntax highlighting](https://viewsourcecode.org/snaptoken/kilo/07.syntaxHighlighting.html)
8.  [Appendices](https://viewsourcecode.org/snaptoken/kilo/08.appendices.html)

  
[竊 back to snaptoken tutorials](https://viewsourcecode.org/snaptoken)

[1.0.0beta11](https://github.com/snaptoken/kilo-tutorial/tree/v1.0.0beta11) ([changelog](https://github.com/snaptoken/kilo-tutorial/blob/master/CHANGELOG.md))
