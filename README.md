# QArt Code Generator

implementation of [QArt Code generator](https://research.swtch.com/qr/draw/) in rust

<img alt="qr code with a pattern that looks like a cat" src="https://github.com/andrewyur/qart/blob/master/mascot.png" width=400/>

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
- create blocks after data has already been mapped
  - iterate through data_bits instead of data_bytes for mapping phase
  - this will allow for removal of calls to set for unsettable bits
- TODOs
- Add a license + LICENSE file
- Compile to WASM and host on gh-pages
  - image scaler + cropper
  - Pixel editor
  - color selector
  - automatic brightness threshold generation
  
## Long Term Itinerary
- benchmarking & optimization
  - there are lots of uses of extend_from_slice, and vector creations
  - the block struct especially is very slow
- publish to crates.io as a binary crate
  - will need a cmd line parser + all the other things
