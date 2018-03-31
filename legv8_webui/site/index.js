var input_div = document.getElementById("input_div");
var output_div = document.getElementById("output_div");

var input = CodeMirror(input_div, {
    value: "ADD X3, X1, X2",
    lineNumbers: true,
    firstLineNumber: 0,
});

var output = CodeMirror(output_div, {
    lineNumbers: true,
    firstLineNumber: 0,
    readOnly: true,
});

request = new XMLHttpRequest();
request.open('GET', 'legv8_webui.wasm');
request.responseType = 'arraybuffer';
request.send();

request.onload = function() {
    var bytes = request.response;
    WebAssembly.instantiate(bytes).then(results => {
        //results.instance.exports.exported_func();
        console.log(results);

        var exports = results.instance.exports;
        var parse_asm = exports.parse_asm;
        var memory = new Uint32Array(results.instance.exports.memory.buffer);

        var asm = input.getValue();
        var pointer = parse_asm(newString(exports, asm));
        var rom = copyCStr(exports, pointer);
        output.setValue(rom);

        input.on("change", function(instance, change) {
            var asm = instance.getValue();
            var pointer = parse_asm(newString(exports, asm));
            var rom = copyCStr(exports, pointer);
            output.setValue(rom);
        });

    });
};


function newString(module, str) {
  const utf8Encoder = new TextEncoder("UTF-8");
  let string_buffer = utf8Encoder.encode(str)
  let len = string_buffer.length
  let ptr = module.alloc(len + 1)

  let memory = new Uint8Array(module.memory.buffer);
  for (i = 0; i < len; i++) {
    memory[ptr + i] = string_buffer[i]
  }

  memory[ptr + len] = 0;

  return ptr
}

function copyCStr(module, ptr) {
  let orig_ptr = ptr;
  const collectCString = function* () {
    let memory = new Uint8Array(module.memory.buffer);
    while (memory[ptr] !== 0) {
      if (memory[ptr] === undefined) { throw new Error("Tried to read undef mem") }
      yield memory[ptr]
      ptr += 1
    }
  }

  const buffer_as_u8 = new Uint8Array(collectCString())
  const utf8Decoder = new TextDecoder("UTF-8");
  const buffer_as_utf8 = utf8Decoder.decode(buffer_as_u8);
  module.dealloc_str(orig_ptr);
  return buffer_as_utf8
}

