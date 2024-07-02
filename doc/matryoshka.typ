#import "@preview/mantys:0.1.4": *

// Vendored because of https://github.com/jneug/typst-mantys/pull/20
#let cmdref(name) = {
  link(cmd-label(name), cmd-(name))
}
// End Vendored

#import "/lib.typ"

#let date = datetime(year: 2024, month: 07, day: 02)

#let matryoshka = package[matryoshka]

#show: mantys.with(
  ..toml("/typst.toml"),
  title: [matryoshka],
  subtitle: [Nested compilers],
  date: datetime.today(),
  abstract: [
    #matryoshka, named after the famous nesting dolls, is a typst compiler as a typst plugin.
    It allows you to compile typst documents form within typst.
    This is especially useful for documentation authors, who might want to display example code and resulting document in their documentation.
    #matryoshka renders typst code to svg and then embeds that svg into your original document.
  ],
  examples-scope: (matryoshka: lib),
)


= About
#matryoshka, named after the famous nesting dolls, is a Typst package that bundles a full Typst compiler as a Typst plugin.
This allows you to render Typst documents from within your Typst documents.
Why would you want to do this?
If you are a documentation author you might want to show Typst source code and the resulting document side-by-side in your documentation.
Without #matryoshka you would have to save your example code into separate files, compile it manually and then finally load both the source code and the generated image from your main document.
#matryoshka simplified this process.
Just write the example code directly into your document and use a `#show` rule to show both the code and the resulting document.

#example()[
  ````typ
    #show <example>: it => {
      set grid.cell(inset: 1em, align: horizon)
      grid(
        columns: 2,
        gutter: 1em,
        grid.cell(stroke: 1pt, it),
        grid.cell(fill: silver, matryoshka.compile(it.text))
      )
    }
    ```typ
    #align(center, text(17pt)[
      *A fluid dynamic model
      for glacier flow*
    ])
    #grid(columns: (1fr, 1fr), align: center)[
      Therese Tungsten \
      #link("mailto:tung@artos.edu")
    ][
      Dr. John Doe \
      #link("mailto:doe@artos.edu")
    ]
    #pagebreak()
    #lorem(100)
    ```<example>
  ````
]




= Usage
== Using MATRYOSHKA

MATRYOSHKA is imported using
#codesnippet[```typ
  #import "@preview/matryoshka:0.1.0"
  ```]

You can then use the #cmdref("compile") and #cmdref("compile-pages") commands to render Typst code.

While #cmdref("compile") returns #dtype("content") directly, #cmdref("compile-pages") returns an #dtype("array"), which can be used when more control over how the pages are displayed is needed.
#codesnippet[```typ
  #matryoshka.compile("= Hello World")
  ```]

Because pages are #doc("visualize/image") elements they are affected by #doc("visualize/image") set and show rules.
#codesnippet[```typ
  #set image(width: 3cm)
  #matryoshka.compile("= Hello World")
  ```]

Note that in contrast to a normal Typst compiler, MATRYOSHKA automatically uses a page height of #value(auto). You can change this using a set rule in the code you want to compile.



== Available Commands

#tidy-module(
  read("/lib.typ"),
  name: "matryoshka",
  include-example-scope: true,
)
