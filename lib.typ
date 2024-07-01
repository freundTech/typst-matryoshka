#let p = plugin("/matryoshka.wasm")

#let compile(source, filesystem: (:), dont-fail: false) = {
  for (key, value) in filesystem {
    if type(value) == str {
      filesystem.at(key) = bytes(value)
    }
  }
  let arg = cbor.encode(
    (
      source: source,
      filesystem: filesystem,
      dont-fail: dont-fail,
    )
  )

  let output = p.compile(arg)
  let result = cbor.decode(output)

  if result.error != none {
    return result.error
  }
  
  result.pages.map(it => image.decode(it)).join()
}

