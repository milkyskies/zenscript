" Vim syntax file for ZenScript (.zs)
" Language: ZenScript

if exists("b:current_syntax")
  finish
endif

" Keywords
syn keyword zsKeyword       const function type export import from
syn keyword zsKeyword       match return await async
syn keyword zsKeyword       if else

" Built-in constructors
syn keyword zsBuiltin       Ok Err Some None

" Boolean literals
syn keyword zsBoolean       true false

" Types
syn keyword zsType          string number bool void Array Option Result

" Operators
syn match   zsOperator      /|>/
syn match   zsOperator      /=>/
syn match   zsOperator      /->/
syn match   zsOperator      /[+\-*/%=<>!&|?]/
syn match   zsOperator      /==/
syn match   zsOperator      /!=/
syn match   zsOperator      />=/
syn match   zsOperator      /<=/
syn match   zsOperator      /&&/
syn match   zsOperator      /||/
syn match   zsOperator      /\.\./

" Numbers
syn match   zsNumber        /\<\d\+\(\.\d\+\)\?\>/

" Strings
syn region  zsString        start=/"/ skip=/\\"/ end=/"/
syn region  zsTemplate      start=/`/ skip=/\\`/ end=/`/ contains=zsInterp
syn region  zsInterp        start=/\${/ end=/}/ contained

" Comments
syn match   zsComment       /\/\/.*/
syn region  zsComment       start=/\/\*/ end=/\*\// contains=zsComment

" JSX
syn region  zsJsxTag        start=/<\z([A-Z][a-zA-Z0-9]*\)/ end=/\/\?>/ contains=zsJsxAttr,zsString
syn match   zsJsxAttr       /\<[a-z][a-zA-Z0-9]*\>/ contained

" Highlights
hi def link zsKeyword       Keyword
hi def link zsBuiltin       Special
hi def link zsBoolean       Boolean
hi def link zsType          Type
hi def link zsOperator      Operator
hi def link zsNumber        Number
hi def link zsString        String
hi def link zsTemplate      String
hi def link zsInterp        Special
hi def link zsComment       Comment
hi def link zsJsxTag        Function
hi def link zsJsxAttr       Identifier

let b:current_syntax = "zenscript"
