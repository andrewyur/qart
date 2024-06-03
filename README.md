# QArt Code Generator

implementation of [QArt Code generator](https://research.swtch.com/qr/draw/) in rust

<img src="https://github.com/andrewyur/qart/blob/master/mascot.png" width=400/>

## Resources
- https://research.swtch.com/field
- https://research.swtch.com/qart
- https://www.thonky.com/qr-code-tutorial/
- https://www.nayuki.io/page/creating-a-qr-code-step-by-step

## Other Implementations
- https://github.com/rsc/qr
- https://github.com/dieforfree/qart4j
- https://github.com/7sDream/pyqart

## Current Itinerary
- separate the image encoder from the qr code
- TODOs
  
## Long Term Itinerary
- benchmarking & optimization
  - there are lots of uses of extend_from_slice, and vector creations
- Compile to WASM and host on gh-pages
  - Pixel editor
  - color selector
  - remove image dependency for smaller package?
- publish to crates.io as a binary crate
  - will need a cmd line parser + all the other things
