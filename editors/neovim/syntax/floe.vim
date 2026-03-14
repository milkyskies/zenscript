" Vim syntax file for Floe (.fl)
" Language: Floe

if exists("b:current_syntax")
  finish
endif

" Keywords
syn keyword floeKeyword       const function type export import from
syn keyword floeKeyword       match return await async
syn keyword floeKeyword       if else

" Built-in constructors
syn keyword floeBuiltin       Ok Err Some None

" Boolean literals
syn keyword zsBoolean       true false

" Types
syn keyword floeType          string number bool void Array Option Result

" Operators
syn match   floeOperator      /|>/
syn match   floeOperator      /=>/
syn match   floeOperator      /->/
syn match   floeOperator      /[+\-*/%=<>!&|?]/
syn match   floeOperator      /==/
syn match   floeOperator      /!=/
syn match   floeOperator      />=/
syn match   floeOperator      /<=/
syn match   floeOperator      /&&/
syn match   floeOperator      /||/
syn match   floeOperator      /\.\./

" Numbers
syn match   floeNumber        /\<\d\+\(\.\d\+\)\?\>/

" Strings
syn region  floeString        start=/"/ skip=/\\"/ end=/"/
syn region  zsTemplate      start=/`/ skip=/\\`/ end=/`/ contains=zsInterp
syn region  zsInterp        start=/\${/ end=/}/ contained

" Comments
syn match   floeComment       /\/\/.*/
syn region  floeComment       start=/\/\*/ end=/\*\// contains=floeComment

" JSX
syn region  zsJsxTag        start=/<\z([A-Z][a-zA-Z0-9]*\)/ end=/\/\?>/ contains=zsJsxAttr,floeString
syn match   zsJsxAttr       /\<[a-z][a-zA-Z0-9]*\>/ contained

" Highlights
hi def link floeKeyword       Keyword
hi def link floeBuiltin       Special
hi def link zsBoolean       Boolean
hi def link floeType          Type
hi def link floeOperator      Operator
hi def link floeNumber        Number
hi def link floeString        String
hi def link zsTemplate      String
hi def link zsInterp        Special
hi def link floeComment       Comment
hi def link zsJsxTag        Function
hi def link zsJsxAttr       Identifier

let b:current_syntax = "floe"
