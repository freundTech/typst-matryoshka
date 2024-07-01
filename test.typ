#import "/lib.typ": compile

#show <example>: it => {
  set grid.cell(inset: 1em, align: horizon)
  grid(
    columns: 2,
    gutter: 1em,
    grid.cell(stroke: 1pt, it),
    grid.cell(fill: gray, compile(it.text, dont-fail: false, filesystem: ("content.typ": "#lorem(100)", "lib.typ": read("lib.typ"), "matryoshka.wasm": read("matryoshka.wasm", encoding: none)))),
  )
}

```typ
#align(center, text(17pt)[
  *A fluid dynamic model
  for glacier flow*
])

#grid(
  columns: (1fr, 1fr),
  align(center)[
    Therese Tungsten \
    Artos Institute \
    #link("mailto:tung@artos.edu")
  ],
  align(center)[
    Dr. John Doe \
    Artos Institute \
    #link("mailto:doe@artos.edu")
  ]
)

#pagebreak()

#include "content.typ"

#pagebreak()

#import "/lib.typ": compile

#block(fill: gray, radius: 1em, inset: 1em)[
  #compile("
  = Hello world

  This is typst in typst in typst
  ")
]
```<example>
