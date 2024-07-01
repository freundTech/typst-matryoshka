#let p = plugin("./target/wasm32-unknown-unknown/release/typst_matryoshka.wasm")

#show <example>: it => {
  let res = p.compile(bytes(it.text))

  set grid.cell(inset: 1em, align: horizon)
  grid(
    columns: 2,
    gutter: 1em,
    grid.cell(stroke: 1pt, it),
    grid.cell(fill: gray, image.decode(res, format: "svg")),
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
```<example>