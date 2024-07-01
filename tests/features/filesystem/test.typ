#import "/lib.typ": compile
#set page(fill: gray)

#compile("#include \"file.typ\"", filesystem: ("file.typ": "Hello World"))
