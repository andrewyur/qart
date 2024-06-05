# QArt Code Generator

implementation of [QArt Code generator](https://research.swtch.com/qr/draw/) in rust

<img alt="qr code with a pattern that looks like a cat" src="https://github.com/andrewyur/qart/blob/master/mascot.png" width=400/>

## Usage
if you would like to see a preview of what aspects of your image will be preserved in the code/fiddle with the brightness threshold without waiting for a code to generate, use the preview function.

if you want a full, scannable qr code to the link you provided, use the build function. see main.rs for complete details.

## Tips
high contrast images work well, and drawings in ms paint work [particularly well](https://github.com/andrewyur/qart/blob/master/mascot2.png), although images with a bad light/dark balance often dont work at the highest qr code sizes.

the brightness threshold value is similar to exposure in photography, except inverted: a higher value means more pixels will be black.

these qr codes have a low error correction level to allow for more drawing room, so scanners often need a bit more time and finagling to scan (depending on the code)

the way this crate manipulates the qr code appearance while still maintaining their functionality is by appending a url fragment after the supplied target text, so unfortunately if you want to encode just text, or any other data besides a url, there will be a massive string of numbers after the target data. read more about the process [here](https://research.swtch.com/qart), where the creator of the process, Russ Cox (of Go fame), describes it in further detail. However, the article does not cover the technical details, and if you are looking to make your own implementation, I tried to document the process as well as practicality allowed for so feel free to take a look inside.

## Current Itinerary
- Add a LICENSE file
- compare performance to other implementations
- Compile to WASM and host on gh-pages (separate repo)
  - image scaler + cropper
  - Pixel editor
  - color selector
  - automatic brightness threshold generation
  
## Long Term Itinerary
- benchmarking & optimization
  - there are lots of uses of extend_from_slice, and vector creations
  - the block struct especially is very slow
- publish to crates.io as a binary crate
  - will need a cmd line parser and cleaner code, at the least

## A Note on the TODOs
The only things left on the TODO list are restructuring and potential optimization. I am new to rust, so I am not sure to what extent the code is optimized by the compiler, so minor things like removing unused vector allocations and caching are on there. At this point, attempting to make (technical) optimizations is starting take a lot longer, and have diminishing returns (the last attempted optimization broke the code, and also made it slower so i decided it wasnt worth it). Since I don't really care if this crate is used by anyone else, continuing to work on todos is not going to be benefiting anyone much, so I have decided to call it here, and pivot to working on other projects.

## Resources
- https://research.swtch.com/field
- https://research.swtch.com/qart
- https://www.thonky.com/qr-code-tutorial/
- https://www.nayuki.io/page/creating-a-qr-code-step-by-step

## Other Implementations
- https://github.com/rsc/qr
- https://github.com/dieforfree/qart4j
- https://github.com/7sDream/pyqart