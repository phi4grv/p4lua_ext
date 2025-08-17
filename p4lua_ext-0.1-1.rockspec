rockspec_format = "3.0"
package = "p4lua_ext"
version = "0.1-1"
source = {
   url = "git+https://github.com/phi4grv/p4lua_ext.git"
}
description = {
   homepage = "https://github.com/phi4grv/p4lua_ext",
   license = "MIT"
}
dependencies = {
   "lua >= 5.1, < 5.5",
}
build_dependencies = {
   "luarocks-build-rust-mlua",
}
build = {
   type = "rust-mlua",
   modules = {
      ["p4lua_ext"] = "p4lua_ext"
   },
}
test_dependencies = {
   "busted >= 2.2.0",
}
test = {
   type = "busted",
}
